use anyhow::{bail, Context};
use clap::ValueEnum;
use log::*;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Deref;
use svd_parser::svd::{self, MaybeArray, PeripheralInfo, RegisterInfo};

use crate::{ir::*, transform};

#[derive(Debug)]
struct ProtoBlock {
    name: Vec<String>,
    description: Option<String>,
    registers: Vec<svd::RegisterCluster>,
}

#[derive(Debug)]
struct ProtoFieldset {
    name: Vec<String>,
    description: Option<String>,
    bit_size: u32,
    // The fields of this proto, and the name of
    // the enum that makes up this field, if it has one.
    fields: Vec<(svd::Field, Option<Vec<String>>)>,
}

#[derive(Debug)]
struct ProtoEnum {
    name: Vec<String>,
    bit_size: u32,
    variants: Vec<svd::EnumeratedValue>,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum NamespaceMode {
    None,
    Block,
    BlockWithRegsVals,
}

fn remove_placeholder(str: &str) -> String {
    str.replace("[%s]", "").replace("%s", "")
}

fn names<T, F>(array: &MaybeArray<T>, f: F) -> ExpandedMaybeArray
where
    F: Fn(&T) -> &str,
{
    fn is_numeric(str: &String) -> bool {
        str.chars().all(|v| v.is_numeric())
    }

    fn replace_placeholder(str: &str, replacement: &str) -> String {
        str.replace("%s", replacement)
    }

    fn as_array_name<'a>(
        r: &str,
        mut dim_element: impl Iterator<Item = &'a String>,
    ) -> Option<&str> {
        if let Some(array) = r.strip_suffix("[%s]") {
            Some(array)
        } else if let Some(missed_array) = r.strip_suffix("%s") {
            // If all dimensions are numeric, the element is an IR
            // array because accessing with a number offset makes
            // sense.
            if dim_element.all(is_numeric) {
                Some(missed_array)
            } else {
                None
            }
        } else {
            None
        }
    }

    match array {
        MaybeArray::Single(r) => ExpandedMaybeArray::Single(f(r).to_string()),
        MaybeArray::Array(r, dim_element) => {
            if let Some(array_name) = as_array_name(f(r), dim_element.dim_index.iter().flatten()) {
                ExpandedMaybeArray::Array {
                    name: array_name.to_string(),
                    array: Array::Regular(RegularArray {
                        len: dim_element.dim,
                        stride: dim_element.dim_increment,
                    }),
                }
            } else {
                let offsets = (0..).step_by(dim_element.dim_increment as _);

                let values = offsets
                    .zip(
                        dim_element
                            .dim_index
                            .iter()
                            .flat_map(|v| v.iter())
                            .map(|dim| replace_placeholder(f(r), dim)),
                    )
                    .collect();

                ExpandedMaybeArray::Many(values)
            }
        }
    }
}

#[derive(Clone)]
enum ExpandedMaybeArray {
    Single(String),
    Array { name: String, array: Array },
    Many(Vec<(u32, String)>),
}

impl ExpandedMaybeArray {
    pub fn array(&self) -> Option<Array> {
        match self {
            ExpandedMaybeArray::Array { array, .. } => Some(array.clone()),
            _ => None,
        }
    }
}

impl IntoIterator for ExpandedMaybeArray {
    type Item = (u32, String);

    type IntoIter = std::vec::IntoIter<(u32, String)>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ExpandedMaybeArray::Single(s) => vec![(0, s)].into_iter(),
            ExpandedMaybeArray::Array { name, .. } => vec![(0, name)].into_iter(),
            ExpandedMaybeArray::Many(items) => items.into_iter(),
        }
    }
}

fn fieldset_name(mut block_name: Vec<String>, reg_name: String) -> Vec<String> {
    block_name.push(reg_name);
    block_name
}

pub fn convert_peripheral(ir: &mut IR, p: &svd::Peripheral) -> anyhow::Result<()> {
    let mut blocks = Vec::new();
    let pname = p.header_struct_name.clone().unwrap_or(p.name.clone());
    collect_blocks(
        &mut blocks,
        vec![pname],
        p.description.clone(),
        p.registers.as_deref().unwrap_or(&[]),
    );

    let enum_from_name = enum_map(&blocks);

    let mut fieldsets: Vec<ProtoFieldset> = Vec::new();
    let mut enums: Vec<ProtoEnum> = Vec::new();
    // A map mapping fully expanded fieldset names to their unique
    // equivalent. E.g. `blocka::REG%s` with dimensions `A`, `B`, `C` would
    // add `blocka::REGA -> blocka::REG`, `blocka::REGB -> blocka::REG` and
    // `blocka::REGC -> REG` to this map.
    let mut fieldset_mapping = BTreeMap::new();
    let mut used_fieldset_names = BTreeSet::new();

    let usable_register_clusters = blocks
        .iter()
        .flat_map(|b| std::iter::repeat(&b.name).zip(b.registers.iter()))
        .filter_map(|(b, r)| {
            let svd::RegisterCluster::Register(r) = r else {
                return None;
            };

            if r.derived_from.is_some() {
                return None;
            }

            let Some(fields) = r.fields.as_ref() else {
                return None;
            };

            let without_placeholder = remove_placeholder(&r.name);
            let mut unique_fs_name = fieldset_name(b.clone(), without_placeholder.clone());

            let mut counter = 0;
            loop {
                if counter != 0 {
                    let last = unique_fs_name.last_mut().unwrap();
                    if last.ends_with('_') {
                        *last = format!("{}{}", last, counter);
                    } else {
                        *last = format!("{}_{}", without_placeholder, counter);
                    }
                }

                counter += 1;

                if !used_fieldset_names.contains(&unique_fs_name) {
                    break;
                }
            }

            used_fieldset_names.insert(unique_fs_name.clone());

            for (_, full_name) in names(r, |r| &r.name) {
                let full_name: Vec<_> = fieldset_name(b.clone(), full_name);
                fieldset_mapping.insert(full_name, unique_fs_name.clone());
            }

            Some((unique_fs_name, r.deref(), fields))
        });

    for (fieldset_name, r, fields) in usable_register_clusters {
        let mut field_name_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut out_fields = Vec::with_capacity(fields.len());

        for f in fields {
            if f.derived_from.is_some() {
                continue;
            }

            let mut enum_read = None;
            let mut enum_write = None;
            let mut enum_readwrite = None;

            let mut field_name = remove_placeholder(&f.name);
            let field_name_count = field_name_counts.entry(field_name.clone()).or_insert(0);
            *field_name_count += 1;
            if *field_name_count > 1 {
                field_name = format!("{}{}", field_name, field_name_count);
            }

            for e in &f.enumerated_values {
                let e = if let Some(derived_from) = &e.derived_from {
                    let Some(e) = enum_from_name.get(derived_from.as_str()) else {
                        warn!(
                            "unknown enum to derive from ({} -> {})",
                            field_name, derived_from
                        );
                        continue;
                    };
                    e
                } else {
                    e
                };

                let usage = e.usage.unwrap_or(svd::Usage::ReadWrite);
                let target = match usage {
                    svd::Usage::Read => &mut enum_read,
                    svd::Usage::Write => &mut enum_write,
                    svd::Usage::ReadWrite => &mut enum_readwrite,
                };

                if target.is_some() {
                    warn!("ignoring enum with dup usage {:?}", usage);
                    continue;
                }

                *target = Some(e)
            }

            enum EnumSet<'a> {
                Single(&'a svd::EnumeratedValues),
                ReadWrite(&'a svd::EnumeratedValues, &'a svd::EnumeratedValues),
            }

            let set = match (enum_read, enum_write, enum_readwrite) {
                (None, None, None) => None,
                (Some(e), None, None) => Some(EnumSet::Single(e)),
                (None, Some(e), None) => Some(EnumSet::Single(e)),
                (None, None, Some(e)) => Some(EnumSet::Single(e)),
                (Some(r), Some(w), None) => Some(EnumSet::ReadWrite(r, w)),
                (Some(r), None, Some(w)) => Some(EnumSet::ReadWrite(r, w)),
                (None, Some(w), Some(r)) => Some(EnumSet::ReadWrite(r, w)),
                (Some(_), Some(_), Some(_)) => {
                    bail!("cannot have enumeratedvalues for read, write and readwrite!")
                }
            };

            let enumm = if let Some(set) = set {
                let variants = match set {
                    EnumSet::Single(e) => e.values.clone(),
                    EnumSet::ReadWrite(r, w) => {
                        let r_values = r.values.iter().map(|v| v.value.unwrap());
                        let w_values = w.values.iter().map(|v| v.value.unwrap());
                        let values: BTreeSet<_> = r_values.chain(w_values).collect();
                        let mut values: Vec<_> = values.iter().collect();
                        values.sort();

                        let r_values: BTreeMap<_, _> =
                            r.values.iter().map(|v| (v.value.unwrap(), v)).collect();
                        let w_values: BTreeMap<_, _> =
                            w.values.iter().map(|v| (v.value.unwrap(), v)).collect();

                        values
                            .into_iter()
                            .map(|&v| match (r_values.get(&v), w_values.get(&v)) {
                                (None, None) => unreachable!(),
                                (Some(&r), None) => r.clone(),
                                (None, Some(&w)) => w.clone(),
                                (Some(&r), Some(&w)) => {
                                    let mut m = r.clone();
                                    if r.name != w.name {
                                        m.name = format!("R_{}_W_{}", r.name, w.name);
                                    }
                                    m
                                }
                            })
                            .collect()
                    }
                };

                let mut name = fieldset_name.clone();
                name.push(field_name);
                enums.push(ProtoEnum {
                    name: name.clone(),
                    bit_size: f.bit_range.width,
                    variants,
                });
                Some(name)
            } else {
                None
            };

            out_fields.push((f.clone(), enumm));
        }

        fieldsets.push(ProtoFieldset {
            name: fieldset_name.clone(),
            description: r.description.clone(),
            bit_size: r.properties.size.unwrap_or(32),
            fields: out_fields,
        });
    }

    // Make all collected names unique by prefixing with parents' names if needed.
    let block_names = unique_names(blocks.iter().map(|x| x.name.clone()).collect())?;
    let fieldset_names = unique_names(fieldsets.iter().map(|x| x.name.clone()).collect())?;
    let enum_names = unique_names(enums.iter().map(|x| x.name.clone()).collect())?;

    // Convert blocks
    for proto in &blocks {
        let mut block = Block {
            extends: None,
            description: proto.description.clone(),
            items: Vec::new(),
        };

        for r in &proto.registers {
            match r {
                svd::RegisterCluster::Register(r) => {
                    if r.derived_from.is_some() {
                        warn!("unsupported derived_from in registers");
                        continue;
                    }

                    let names = crate::svd2ir::names(&r, |r| &r.name);
                    let array = names.array();

                    for (offset, name) in names {
                        let block_item = create_block_item(
                            name,
                            offset,
                            proto,
                            &fieldset_mapping,
                            &fieldset_names,
                            array.as_ref(),
                            r,
                        );
                        block.items.push(block_item)
                    }
                }
                svd::RegisterCluster::Cluster(c) => {
                    if c.derived_from.is_some() {
                        warn!("unsupported derived_from in clusters");
                        continue;
                    }

                    let names = names(c, |c| &c.name);
                    let array = names.array();

                    for (offset, cname) in names {
                        let mut block_name = proto.name.clone();
                        block_name.push(cname.clone());
                        let block_name = block_names.get(&block_name).unwrap().clone();

                        block.items.push(BlockItem {
                            name: cname.clone(),
                            description: c.description.clone(),
                            array: array.clone(),
                            byte_offset: c.address_offset + offset,
                            inner: BlockItemInner::Block(BlockItemBlock { block: block_name }),
                        });
                    }
                }
            }
        }

        let block_name = block_names.get(&proto.name).unwrap().clone();
        assert!(ir.blocks.insert(block_name, block).is_none())
    }

    // Convert fieldsets
    for proto in &fieldsets {
        let mut fieldset = FieldSet {
            extends: None,
            description: proto.description.clone(),
            bit_size: proto.bit_size,
            fields: Vec::new(),
        };

        for (f, enumm) in &proto.fields {
            if f.derived_from.is_some() {
                warn!("unsupported derived_from in fieldset");
            }

            let names = names(f, |f| &f.name);
            let array = names.array();

            for (offset, field_name) in names {
                let field = Field {
                    name: field_name.clone(),
                    description: f.description.clone(),
                    bit_offset: BitOffset::Regular(f.bit_range.offset + offset),
                    bit_size: f.bit_range.width,
                    array: array.clone(),
                    enumm: enumm.as_ref().map(|v| {
                        enum_names
                            .get(v)
                            .cloned()
                            .expect("All enums have a unique-name mapping")
                    }),
                };

                fieldset.fields.push(field)
            }
        }

        let fieldset_name = fieldset_names.get(&proto.name).unwrap().clone();
        assert!(ir.fieldsets.insert(fieldset_name, fieldset).is_none())
    }

    for proto in &enums {
        let variants = proto
            .variants
            .iter()
            .filter_map(|v| {
                // A value which is default and has no defined value should be skipped.
                if v.is_default() && v.value.is_none() {
                    return None;
                }

                Some(EnumVariant {
                    description: v.description.clone(),
                    name: v.name.clone(),
                    value: v.value.unwrap() as _, // TODO what are variants without values used for??
                })
            })
            .collect();

        let enumm = Enum {
            description: None,
            bit_size: proto.bit_size,
            variants,
        };

        let enum_name = enum_names.get(&proto.name).unwrap().clone();
        assert!(ir.enums.insert(enum_name.clone(), enumm).is_none());
    }

    Ok(())
}

fn create_block_item(
    name: String,
    offset: u32,
    proto: &ProtoBlock,
    inner_block_to_fieldset: &BTreeMap<Vec<String>, Vec<String>>,
    fieldset_names: &BTreeMap<Vec<String>, String>,
    array: Option<&Array>,
    r: &MaybeArray<RegisterInfo>,
) -> BlockItem {
    let fieldset_name = if r.fields.is_some() {
        let fieldset_full_name = fieldset_name(proto.name.clone(), name.clone());
        let fieldset_name = inner_block_to_fieldset.get(&fieldset_full_name).unwrap();
        Some(fieldset_names.get(fieldset_name).unwrap().clone())
    } else {
        None
    };

    let access = match r.properties.access {
        None => Access::ReadWrite,
        Some(svd::Access::ReadOnly) => Access::Read,
        Some(svd::Access::WriteOnly) => Access::Write,
        Some(svd::Access::WriteOnce) => Access::Write,
        Some(svd::Access::ReadWrite) => Access::ReadWrite,
        Some(svd::Access::ReadWriteOnce) => Access::ReadWrite,
    };

    BlockItem {
        name: name,
        description: r.description.clone(),
        array: array.cloned(),
        byte_offset: r.address_offset + offset,
        inner: BlockItemInner::Register(Register {
            access, // todo
            bit_size: r.properties.size.unwrap_or(32),
            fieldset: fieldset_name.clone(),
        }),
    }
}

/// Convert an entire SVD to IR.
pub fn convert_svd(svd: &svd::Device, namespace: NamespaceMode) -> anyhow::Result<IR> {
    let mut ir = IR::new();

    let mut device = Device {
        nvic_priority_bits: svd.cpu.as_ref().map(|cpu| cpu.nvic_priority_bits as u8),
        peripherals: vec![],
        interrupts: vec![],
    };

    for p in &svd.peripherals {
        let base_p = if let Some(derived) = &p.derived_from {
            svd.peripherals.iter().find(|p| p.name == *derived).unwrap()
        } else {
            p
        };
        let block_name = base_p
            .header_struct_name
            .clone()
            .unwrap_or(base_p.name.clone());
        let block_path = format!("{}::{}", block_name, block_name);
        let peri_name = p.name.to_ascii_uppercase();

        let peri = Peripheral {
            name: peri_name.clone(),
            description: p.description.clone(),
            base_address: p.base_address,
            block: Some(block_path),
            array: None,
            interrupts: BTreeMap::new(),
        };

        let mut irqs: Vec<&svd::Interrupt> = vec![];
        for i in &p.interrupt {
            if !irqs.iter().any(|&j| j.name == i.name) {
                irqs.push(i)
            }
        }
        irqs.sort_by_key(|i| &i.name);

        for &i in irqs.iter() {
            let iname = i.name.to_ascii_uppercase();

            if !device.interrupts.iter().any(|j| j.name == iname) {
                device.interrupts.push(Interrupt {
                    name: iname.clone(),
                    description: i.description.clone(),
                    value: i.value,
                });
            }

            /*
            let name = if iname.len() > periname.len() && iname.starts_with(&periname) {
                let s = iname.strip_prefix(&periname).unwrap();
                s.trim_matches('_').to_string()
            } else if irqs.len() == 1 {
                "IRQ".to_string()
            } else {
                format!("IRQ{}", n)
            };

            peri.interrupts.insert(name, iname.clone());
             */
        }

        device.peripherals.push(peri);

        if p.derived_from.is_none() {
            let mut pir = IR::new();
            convert_peripheral(&mut pir, p)?;
            namespace_names(p, &mut pir, namespace);
            ir.merge(pir);
        }
    }

    ir.devices.insert("".to_string(), device);

    transform::sort::Sort {}.run(&mut ir).unwrap();

    Ok(ir)
}

/// Create a map of all enums by name.
/// Ignores potential duplicates of names.
fn enum_map(blocks: &[ProtoBlock]) -> BTreeMap<&'_ str, &'_ svd::EnumeratedValues> {
    let mut map = BTreeMap::new();
    for block in blocks {
        for r in &block.registers {
            let svd::RegisterCluster::Register(r) = r else {
                continue;
            };
            if r.derived_from.is_some() {
                continue;
            }
            let Some(fields) = &r.fields else { continue };
            for f in fields {
                for e in &f.enumerated_values {
                    if let Some(name) = &e.name {
                        map.insert(name.as_str(), e);
                    }
                }
            }
        }
    }
    map
}

fn collect_blocks(
    out: &mut Vec<ProtoBlock>,
    block_name: Vec<String>,
    description: Option<String>,
    registers: &[svd::RegisterCluster],
) {
    out.push(ProtoBlock {
        name: block_name.clone(),
        description,
        registers: registers.to_owned(),
    });

    for r in registers {
        if let svd::RegisterCluster::Cluster(c) = r {
            if c.derived_from.is_some() {
                continue;
            }

            for (_, block) in names(c, |c| &c.name) {
                let mut block_name = block_name.clone();
                block_name.push(block);
                collect_blocks(out, block_name, c.description.clone(), &c.children);
            }
        }
    }
}

/// An iterator yielding successively longer suffixes
fn suffixes(n: &[String]) -> impl Iterator<Item = &[String]> {
    (0..n.len()).rev().map(|v| &n[v..])
}

fn unique_names(names: Vec<Vec<String>>) -> anyhow::Result<BTreeMap<Vec<String>, String>> {
    let names2 = names
        .iter()
        .map(|n| {
            // asfd
            let mut res = Vec::new();
            let mut prefix = String::new();
            for s in n.iter() {
                if s == "PSEL" {
                    if !prefix.is_empty() {
                        prefix.push('_');
                    }
                    prefix.push_str(s);
                } else if prefix.is_empty() {
                    res.push(s.clone());
                } else {
                    res.push(format!("{prefix}_{s}"));
                    prefix = String::new()
                }
            }
            if !prefix.is_empty() {
                res.push(prefix);
            }
            res
        })
        .collect::<Vec<_>>();

    let mut res = BTreeMap::new();
    let mut suffix_occurrences = BTreeMap::new();

    for name in names2.iter() {
        for suffix in suffixes(&name) {
            let entry = suffix_occurrences.entry(suffix).or_insert(0);
            *entry += 1;
        }
    }

    for (original, n) in names.iter().zip(names2.iter()) {
        let shortest_unique_suffix = suffixes(n).find(|suffix| {
            let suffix_is_unique = *suffix_occurrences.get(suffix).unwrap() == 1;
            suffix_is_unique
        });

        let suffix = shortest_unique_suffix.with_context(|| {
            format!("Failed to find unique name for {:?}. n: {:?}", original, n)
        })?;

        assert!(res.insert(original.clone(), suffix.join("_")).is_none());
    }
    Ok(res)
}

/// Derive a canonical block name from a SVD peripheral.
fn block_name(peripheral: &PeripheralInfo) -> String {
    peripheral
        .header_struct_name
        .clone()
        .unwrap_or(peripheral.name.clone())
}

/// Map all IR objects to a block namespace.
///
/// Optionally add the '::regs' for registers and '::vals' for enums submodules.
pub fn namespace_names(peripheral: &PeripheralInfo, ir: &mut IR, namespace: NamespaceMode) {
    let block_name = block_name(peripheral);

    transform::map_names(ir, |k, s| {
        // Denotes whether to transform the name, and with which (empty) midfix.
        let transform_midfix: Option<&str> = match k {
            transform::NameKind::Block => Some(""),
            transform::NameKind::Fieldset => Some("::regs"),
            transform::NameKind::Enum => Some("::vals"),
            _ => None,
        };

        if let Some(midfix) = transform_midfix {
            match namespace {
                NamespaceMode::None => (), // Do nothing.
                NamespaceMode::Block => *s = format!("{}::{}", block_name, s),
                NamespaceMode::BlockWithRegsVals => *s = format!("{}{}::{}", block_name, midfix, s),
            }
        }
    });
}
