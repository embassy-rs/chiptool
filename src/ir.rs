use de::MapAccess;
use serde::{de, de::Visitor, ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::ops::RangeInclusive;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct IR {
    pub devices: HashMap<String, Device>,
    pub blocks: HashMap<String, Block>,
    pub fieldsets: HashMap<String, FieldSet>,
    pub enums: HashMap<String, Enum>,
}

impl IR {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge(&mut self, other: IR) {
        self.devices.extend(other.devices);
        self.blocks.extend(other.blocks);
        self.fieldsets.extend(other.fieldsets);
        self.enums.extend(other.enums);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nvic_priority_bits: Option<u8>,
    pub peripherals: Vec<Peripheral>,
    pub interrupts: Vec<Interrupt>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Peripheral {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub base_address: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub array: Option<Array>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,

    #[serde(
        default,
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "ordered_map"
    )]
    pub interrupts: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Interrupt {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extends: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub items: Vec<BlockItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockItem {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub array: Option<Array>,
    pub byte_offset: u32,

    #[serde(flatten)]
    pub inner: BlockItemInner,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockItemInner {
    Block(BlockItemBlock),
    Register(Register),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Array {
    Regular(RegularArray),
    Cursed(CursedArray),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegularArray {
    pub len: u32,
    pub stride: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CursedArray {
    pub offsets: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Register {
    #[serde(default = "default_readwrite", skip_serializing_if = "is_readwrite")]
    pub access: Access,
    #[serde(default = "default_32", skip_serializing_if = "is_32")]
    pub bit_size: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fieldset: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockItemBlock {
    pub block: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Access {
    ReadWrite,
    Read,
    Write,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extends: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "default_32", skip_serializing_if = "is_32")]
    pub bit_size: u32,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(untagged)]
pub enum BitOffset {
    Regular(u32),
    // This vector assume all RangeInclusive is non-overlapped and sorted.
    // It should be checked when parse source files.
    Cursed(Vec<RangeInclusive<u32>>),
}

impl BitOffset {
    pub(crate) fn min_offset(&self) -> u32 {
        match self {
            BitOffset::Regular(offset) => *offset,
            BitOffset::Cursed(range_list) => *range_list[0].start(),
        }
    }

    pub(crate) fn max_offset(&self) -> u32 {
        match self {
            BitOffset::Regular(offset) => *offset,
            BitOffset::Cursed(range_list) => *range_list[range_list.len()].end(),
        }
    }

    pub(crate) fn into_ranges(self, bit_size: u32) -> Vec<RangeInclusive<u32>> {
        match self {
            BitOffset::Regular(offset) => vec![offset..=offset + bit_size - 1],
            BitOffset::Cursed(ranges) => ranges,
        }
    }
}

// custom bit offset ordering:
// 1. compare min offset: less is less, greater is greater
// 2. when min offset is equal, compare max offset: less is less, greater is greater, equal is equal
impl PartialOrd for BitOffset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        let min_order = self.min_offset().cmp(&other.min_offset());
        let result = match min_order {
            Ordering::Equal => self.max_offset().cmp(&other.max_offset()),
            min_order => min_order,
        };

        Some(result)
    }
}

impl Ord for BitOffset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub bit_offset: BitOffset,
    pub bit_size: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub array: Option<Array>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enum")]
    pub enumm: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Enum {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub bit_size: u32,
    pub variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: u64,
}

fn default_32() -> u32 {
    32
}
fn is_32(x: &u32) -> bool {
    *x == 32
}

fn default_readwrite() -> Access {
    Access::ReadWrite
}
fn is_readwrite(x: &Access) -> bool {
    *x == Access::ReadWrite
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Block,
    Fieldset,
    Enum,
}

impl Serialize for IR {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Sort by block/fieldset/enum, then alphabetically.
        // This ensures the output's order is deterministic.
        // - Easier diffing between yamls
        // - No spurious changes when roundtripping
        let mut entries = Vec::new();
        for name in self.blocks.keys() {
            entries.push((Kind::Block, name));
        }
        for name in self.fieldsets.keys() {
            entries.push((Kind::Fieldset, name));
        }
        for name in self.enums.keys() {
            entries.push((Kind::Enum, name));
        }

        entries.sort();

        let mut map = serializer.serialize_map(Some(entries.len()))?;
        for (kind, name) in entries {
            match kind {
                Kind::Block => {
                    map.serialize_entry(
                        &format!("block/{}", name),
                        self.blocks.get(name).unwrap(),
                    )?;
                }
                Kind::Fieldset => {
                    map.serialize_entry(
                        &format!("fieldset/{}", name),
                        self.fieldsets.get(name).unwrap(),
                    )?;
                }
                Kind::Enum => {
                    map.serialize_entry(&format!("enum/{}", name), self.enums.get(name).unwrap())?;
                }
            }
        }
        map.end()
    }
}

struct IRVisitor;

impl<'de> Visitor<'de> for IRVisitor {
    type Value = IR;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IR")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut ir = IR::new();

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some(key) = access.next_key()? {
            let key: String = key;
            let (kind, name) = key.split_once('/').ok_or(de::Error::custom("item names must be in form `kind/name`, where kind is `block`, `fieldset` or `enum`"))?;
            match kind {
                "block" => {
                    let val: Block = access.next_value()?;
                    if ir.blocks.insert(name.to_string(), val).is_some() {
                        return Err(de::Error::custom(format!("Duplicate item {:?}", key)));
                    }
                }
                "fieldset" => {
                    let val: FieldSet = access.next_value()?;
                    if ir.fieldsets.insert(name.to_string(), val).is_some() {
                        return Err(de::Error::custom(format!("Duplicate item {:?}", key)));
                    }
                }
                "enum" => {
                    let val: Enum = access.next_value()?;
                    if ir.enums.insert(name.to_string(), val).is_some() {
                        return Err(de::Error::custom(format!("Duplicate item {:?}", key)));
                    }
                }
                _ => return Err(de::Error::custom(format!("Unknown kind {:?}", kind))),
            }
        }

        Ok(ir)
    }
}

impl<'de> Deserialize<'de> for IR {
    fn deserialize<D>(deserializer: D) -> Result<IR, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(IRVisitor)
    }
}

fn ordered_map<S>(value: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}
