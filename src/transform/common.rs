use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::ir::*;

pub(crate) fn make_regex(r: &str) -> Result<regex::Regex, regex::Error> {
    regex::Regex::new(&format!("^{}$", r))
}

pub(crate) fn mergeable_enums(a: &Enum, b: &Enum) -> bool {
    a.variants == b.variants
}

fn dfalse() -> bool {
    false
}
fn dtrue() -> bool {
    true
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum FieldsetMergeCheck {
    None,
    Layout,
    Names,
    Descriptions,
}

impl Default for FieldsetMergeCheck {
    fn default() -> Self {
        Self::Names
    }
}

pub(crate) fn check_mergeable_fieldsets(
    a: &FieldSet,
    b: &FieldSet,
    check: FieldsetMergeCheck,
) -> anyhow::Result<()> {
    if let Err(e) = check_mergeable_fieldsets_inner(a, b, check) {
        bail!(
            "Cannot merge fieldsets.\nfirst: {:#?}\nsecond: {:#?}\ncause: {:?}",
            a,
            b,
            e
        )
    }
    Ok(())
}

pub(crate) fn check_mergeable_fieldsets_inner(
    a: &FieldSet,
    b: &FieldSet,
    check: FieldsetMergeCheck,
) -> anyhow::Result<()> {
    if a.bit_size != b.bit_size {
        bail!("Different bit size: {} vs {}", a.bit_size, b.bit_size)
    }

    if check >= FieldsetMergeCheck::Layout {
        if a.fields.len() != b.fields.len() {
            bail!("Different field count")
        }

        let field_match = |fa: &Field, fb: &Field| {
            let mut res = fa.bit_size == fb.bit_size && fa.bit_offset == fb.bit_offset;
            if check >= FieldsetMergeCheck::Names {
                res &= fa.name == fb.name;
            }
            if check >= FieldsetMergeCheck::Descriptions {
                res &= fa.description == fb.description;
            }
            res
        };

        let mut aok = [false; 128];
        let mut bok = [false; 128];

        for (ia, fa) in a.fields.iter().enumerate() {
            if let Some((ib, fb)) = b
                .fields
                .iter()
                .enumerate()
                .find(|(ib, fb)| !bok[*ib] && field_match(fa, fb))
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

pub(crate) fn match_paths<T: Pathed>(set: &Set<T>, re: &regex::Regex) -> HashSet<Id<T>> {
    let mut ids: HashSet<Id<T>> = HashSet::new();
    for (id, e) in set.iter() {
        if path_matches(e.path(), &re) {
            ids.insert(id);
        }
    }
    ids
}

pub(crate) fn path_groups<T: Pathed>(
    set: &Set<T>,
    re: &regex::Regex,
    to: &String,
) -> HashMap<Path, HashSet<Id<T>>> {
    let mut groups: HashMap<Path, HashSet<Id<T>>> = HashMap::new();
    for (id, e) in set.iter() {
        if let Some(to) = path_match_expand(e.path(), &re, to) {
            if let Some(v) = groups.get_mut(&to) {
                v.insert(id);
            } else {
                let mut s = HashSet::new();
                s.insert(id);
                groups.insert(to, s);
            }
        }
    }
    groups
}

pub(crate) fn string_groups(
    set: impl Iterator<Item = String>,
    re: &regex::Regex,
    to: &String,
) -> HashMap<String, HashSet<String>> {
    let mut groups: HashMap<String, HashSet<String>> = HashMap::new();
    for s in set {
        if let Some(to) = string_match_expand(&s, &re, to) {
            if let Some(v) = groups.get_mut(&to) {
                v.insert(s);
            } else {
                let mut v = HashSet::new();
                v.insert(s);
                groups.insert(to, v);
            }
        }
    }
    groups
}

pub(crate) fn path_matches(path: &Path, regex: &regex::Regex) -> bool {
    let path = path.to_string();
    regex.is_match(&path)
}

pub(crate) fn path_match_expand(path: &Path, regex: &regex::Regex, res: &str) -> Option<Path> {
    let path = path.to_string();
    let m = regex.captures(&path)?;
    let mut dst = String::new();
    m.expand(res, &mut dst);
    Some(Path::new_from_string(&dst))
}

pub(crate) fn string_match_expand(s: &str, regex: &regex::Regex, res: &str) -> Option<String> {
    let m = regex.captures(&s)?;
    let mut dst = String::new();
    m.expand(res, &mut dst);
    Some(dst)
}

pub(crate) fn replace_enum_ids(ir: &mut IR, from: &HashSet<Id<Enum>>, to: Id<Enum>) {
    for (_, fs) in ir.fieldsets.iter_mut() {
        for f in fs.fields.iter_mut() {
            if let Some(id) = f.enumm {
                if from.contains(&id) {
                    f.enumm = Some(to)
                }
            }
        }
    }
}

pub(crate) fn replace_fieldset_ids(ir: &mut IR, from: &HashSet<Id<FieldSet>>, to: Id<FieldSet>) {
    for (_, b) in ir.blocks.iter_mut() {
        for i in b.items.iter_mut() {
            if let BlockItemInner::Register(r) = &mut i.inner {
                if let Some(id) = r.fieldset {
                    if from.contains(&id) {
                        r.fieldset = Some(to)
                    }
                }
            }
        }
    }
}

pub(crate) fn replace_block_ids(ir: &mut IR, from: &HashSet<Id<Block>>, to: Id<Block>) {
    for (_, d) in ir.devices.iter_mut() {
        for p in d.peripherals.iter_mut() {
            if from.contains(&p.block) {
                p.block = to
            }
        }
    }

    for (_, b) in ir.blocks.iter_mut() {
        for i in b.items.iter_mut() {
            if let BlockItemInner::Block(id) = &i.inner {
                if from.contains(&id) {
                    i.inner = BlockItemInner::Block(to)
                }
            }
        }
    }
}
