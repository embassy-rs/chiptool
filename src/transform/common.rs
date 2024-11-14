use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::ir::*;

#[derive(Debug, Clone)]
pub struct RegexSet {
    include: Vec<regex::Regex>,
    exclude: Vec<regex::Regex>,
}

impl RegexSet {
    pub fn captures<'h>(&self, haystack: &'h str) -> Option<regex::Captures<'h>> {
        for r in &self.exclude {
            if r.is_match(haystack) {
                return None;
            }
        }
        for r in &self.include {
            if let Some(c) = r.captures(haystack) {
                return Some(c);
            }
        }
        None
    }

    pub fn is_match(&self, haystack: &str) -> bool {
        for r in &self.exclude {
            if r.is_match(haystack) {
                return false;
            }
        }
        for r in &self.include {
            if r.is_match(haystack) {
                return true;
            }
        }
        false
    }
}

impl<'de> Deserialize<'de> for RegexSet {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        fn make_regex(r: &str) -> Result<regex::Regex, regex::Error> {
            regex::Regex::new(&format!("^{}$", r))
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum VecOrString {
            One(String),
            Many(Vec<String>),
        }
        impl VecOrString {
            fn regexes(self) -> Vec<regex::Regex> {
                let strs = match self {
                    VecOrString::Many(s) => s,
                    VecOrString::One(s) => vec![s],
                };
                strs.into_iter().map(|s| make_regex(&s).unwrap()).collect()
            }
        }

        impl Default for VecOrString {
            fn default() -> Self {
                Self::Many(vec![])
            }
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Inner {
            String(String),
            Complex {
                include: VecOrString,
                #[serde(default)]
                exclude: VecOrString,
            },
        }

        let x = Inner::deserialize(de)?;
        match x {
            Inner::String(s) => Ok(RegexSet {
                include: vec![make_regex(&s).unwrap()],
                exclude: vec![],
            }),
            Inner::Complex { include, exclude } => Ok(RegexSet {
                include: include.regexes(),
                exclude: exclude.regexes(),
            }),
        }
    }
}

impl Serialize for RegexSet {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum CheckLevel {
    NoCheck,
    Layout,
    Names,
    Descriptions,
}

impl Default for CheckLevel {
    fn default() -> Self {
        Self::Names
    }
}

pub(crate) fn check_mergeable_fieldsets(
    a_name: &str,
    a: &FieldSet,
    b_name: &str,
    b: &FieldSet,
    level: CheckLevel,
) -> anyhow::Result<()> {
    if let Err(e) = check_mergeable_fieldsets_inner(a, b, level) {
        bail!(
            "Cannot merge fieldsets.\nfirst: {} {:#?}\nsecond: {} {:#?}\ncause: {:?}",
            a_name,
            a,
            b_name,
            b,
            e
        )
    }
    Ok(())
}

pub(crate) fn mergeable_fields(a: &Field, b: &Field, level: CheckLevel) -> bool {
    let mut res = true;
    if level >= CheckLevel::Layout {
        res &= a.bit_size == b.bit_size
            && a.bit_offset == b.bit_offset
            && a.enumm == b.enumm
            && a.array == b.array;
    }
    if level >= CheckLevel::Names {
        res &= a.name == b.name;
    }
    if level >= CheckLevel::Descriptions {
        res &= a.description == b.description;
    }
    res
}

pub(crate) fn check_mergeable_fieldsets_inner(
    a: &FieldSet,
    b: &FieldSet,
    level: CheckLevel,
) -> anyhow::Result<()> {
    if a.bit_size != b.bit_size {
        bail!("Different bit size: {} vs {}", a.bit_size, b.bit_size)
    }

    if level >= CheckLevel::Layout {
        if a.fields.len() != b.fields.len() {
            bail!("Different field count")
        }

        let mut aok = [false; 128];
        let mut bok = [false; 128];

        for (ia, fa) in a.fields.iter().enumerate() {
            if let Some((ib, _fb)) = b
                .fields
                .iter()
                .enumerate()
                .find(|(ib, fb)| !bok[*ib] && mergeable_fields(fa, fb, level))
            {
                aok[ia] = true;
                bok[ib] = true;
            } else {
                bail!("Field in first fieldset has no match: {:?}", fa);
            }
        }
    }

    Ok(())
}

pub(crate) fn match_all(set: impl Iterator<Item = String>, re: &RegexSet) -> BTreeSet<String> {
    let mut ids: BTreeSet<String> = BTreeSet::new();
    for id in set {
        if re.is_match(&id) {
            ids.insert(id);
        }
    }
    ids
}

pub(crate) fn match_groups(
    set: impl Iterator<Item = String>,
    re: &RegexSet,
    to: &str,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut groups: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for s in set {
        if let Some(to) = match_expand(&s, re, to) {
            if let Some(v) = groups.get_mut(&to) {
                v.insert(s);
            } else {
                let mut v = BTreeSet::new();
                v.insert(s);
                groups.insert(to, v);
            }
        }
    }
    groups
}

pub(crate) fn match_expand(s: &str, regex: &RegexSet, res: &str) -> Option<String> {
    let m = regex.captures(s)?;
    let mut dst = String::new();
    m.expand(res, &mut dst);
    Some(dst)
}

pub(crate) fn replace_enum_ids(ir: &mut IR, from: &BTreeSet<String>, to: String) {
    for (_, fs) in ir.fieldsets.iter_mut() {
        for f in fs.fields.iter_mut() {
            if let Some(id) = &mut f.enumm {
                if from.contains(id) {
                    *id = to.clone()
                }
            }
        }
    }
}

pub(crate) fn replace_fieldset_ids(ir: &mut IR, from: &BTreeSet<String>, to: String) {
    for (_, b) in ir.blocks.iter_mut() {
        for i in b.items.iter_mut() {
            if let BlockItemInner::Register(r) = &mut i.inner {
                if let Some(id) = &r.fieldset {
                    if from.contains(id) {
                        r.fieldset = Some(to.clone())
                    }
                }
            }
        }
    }
}

pub(crate) fn replace_block_ids(ir: &mut IR, from: &BTreeSet<String>, to: String) {
    for (_, d) in ir.devices.iter_mut() {
        for p in d.peripherals.iter_mut() {
            if let Some(block) = &mut p.block {
                if from.contains(block) {
                    *block = to.clone()
                }
            }
        }
    }

    for (_, b) in ir.blocks.iter_mut() {
        for i in b.items.iter_mut() {
            if let BlockItemInner::Block(bi) = &mut i.inner {
                if from.contains(&bi.block) {
                    bi.block = to.clone()
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum ArrayMode {
    #[default]
    Standard,
    Cursed,
    Holey,
}

pub(crate) fn calc_array(mut offsets: Vec<u32>, mode: ArrayMode) -> anyhow::Result<(u32, Array)> {
    offsets.sort_unstable();

    // Guess stride.
    let start_offset = offsets[0];
    let len = offsets.len() as u32;
    let stride = if len == 1 {
        // If there's only 1 item, we can't know the stride, but it
        // doesn't really matter!
        0
    } else {
        offsets[1] - offsets[0]
    };

    // Check the stride guess is OK

    if offsets
        .iter()
        .enumerate()
        .all(|(n, &i)| i == start_offset + (n as u32) * stride)
    {
        // Array is regular,
        return Ok((
            start_offset,
            Array::Regular(RegularArray {
                len: offsets.len() as _,
                stride,
            }),
        ));
    }

    // Array is irregular, If we wanted a regular array, fail.
    match mode {
        ArrayMode::Standard => {
            bail!("arrayize: items are not evenly spaced. Set `mode: Cursed` to allow index->offset relation to be non-linear, or `mode: Holey` to keep it linear but fill the holes with indexes that won't be valid.")
        }
        ArrayMode::Cursed => {
            for o in &mut offsets {
                *o -= start_offset
            }
            Ok((start_offset, Array::Cursed(CursedArray { offsets })))
        }
        ArrayMode::Holey => {
            let len = (offsets.last().unwrap() - offsets.first().unwrap()) / stride + 1;
            Ok((start_offset, Array::Regular(RegularArray { len, stride })))
        }
    }
}

// filter enum by enum name, then copy variant description
pub(crate) fn extract_variant_desc(
    ir: &IR,
    enum_names: &RegexSet,
    bit_size: Option<u32>,
) -> anyhow::Result<BTreeMap<String, String>> {
    let mut enum_desc_pair: BTreeMap<String, String> = BTreeMap::new();
    for (e_name, e_struct) in ir.enums.iter().filter(|(e_name, e_struct)| {
        bit_size.map_or(true, |s| s == e_struct.bit_size) && enum_names.is_match(e_name)
    }) {
        let variant_desc_str = e_struct.variants.iter().fold(String::new(), |mut acc, v| {
            acc.push_str(
                format!(
                    "{}: {}\n",
                    v.value,
                    v.description.clone().unwrap_or_default()
                )
                .as_str(),
            );
            acc
        });

        enum_desc_pair.insert(e_name.clone(), variant_desc_str);
    }

    Ok(enum_desc_pair)
}

// filter field by enum name, then append corresponding variant description
pub(crate) fn append_variant_desc_to_field(
    ir: &mut IR,
    enum_desc_pair: &BTreeMap<String, String>,
    bit_size: Option<u32>,
) {
    for fs in ir.fieldsets.values_mut() {
        for f in fs
            .fields
            .iter_mut()
            .filter(|f| bit_size.map_or(true, |s| s == f.bit_size) && f.enumm.is_some())
        {
            for (_, desc_string) in enum_desc_pair
                .iter()
                .filter(|(e_name, _)| **e_name == f.enumm.clone().unwrap())
            {
                match &f.description {
                    Some(desc) => {
                        f.description = Some(format!("{}\n{}", desc.clone(), desc_string.clone()))
                    }
                    None => f.description = Some(desc_string.clone()),
                }
            }
        }
    }
}
