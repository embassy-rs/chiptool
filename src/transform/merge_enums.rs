use anyhow::bail;
use log::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeEnums {
    pub from: RegexSet,
    pub to: String,
    pub main: Option<RegexSet>,
    #[serde(default)]
    pub check: CheckLevel,
    #[serde(default)]
    pub skip_unmergeable: bool,
    pub keep_desc: Option<bool>,
}

impl MergeEnums {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        if self.keep_desc.unwrap_or(false) {
            let variant_desc = extract_variant_desc(ir, &self.from, None)?;
            append_variant_desc_to_field(ir, &variant_desc, None);
        }

        let groups = match_groups(ir.enums.keys().cloned(), &self.from, &self.to);

        for (to, group) in groups {
            info!("Merging enums, dest: {}", to);
            for id in &group {
                info!("   {}", id);
            }
            self.merge_enums(ir, group, to, self.main.as_ref())?;
        }

        Ok(())
    }

    fn merge_enums(
        &self,
        ir: &mut IR,
        ids: BTreeSet<String>,
        to: String,
        main: Option<&RegexSet>,
    ) -> anyhow::Result<()> {
        let mut main_id = ids.iter().next().unwrap().clone();
        if let Some(main) = main {
            for id in ids.iter() {
                if main.is_match(id) {
                    main_id = id.clone();
                    break;
                }
            }
        }
        let e = ir.enums.get(&main_id).unwrap().clone();

        for id in &ids {
            let e2 = ir.enums.get(id).unwrap();
            if let Err(e) = check_mergeable_enums(&main_id, &e, id, e2, self.check) {
                if self.skip_unmergeable {
                    info!("skipping: {:?}", to);
                    return Ok(());
                } else {
                    return Err(e);
                }
            }
        }
        for id in &ids {
            ir.enums.remove(id);
        }

        assert!(ir.enums.insert(to.clone(), e).is_none());
        replace_enum_ids(ir, &ids, to);

        Ok(())
    }
}

fn check_mergeable_enums(
    a_id: &str,
    a: &Enum,
    b_id: &str,
    b: &Enum,
    level: CheckLevel,
) -> anyhow::Result<()> {
    if let Err(e) = check_mergeable_enums_inner(a, b, level) {
        bail!("Cannot merge enums.\nfirst: {a_id}\n{a:#?}\nsecond: {b_id}\n{b:#?}\ncause: {e:?}",)
    }
    Ok(())
}

fn check_mergeable_enums_inner(a: &Enum, b: &Enum, level: CheckLevel) -> anyhow::Result<()> {
    if a.bit_size != b.bit_size {
        bail!("Different bit size: {} vs {}", a.bit_size, b.bit_size)
    }

    if level >= CheckLevel::Layout {
        if a.variants.len() != b.variants.len() {
            bail!("Different variant count")
        }

        let mut aok = [false; 1024];
        let mut bok = [false; 1024];

        for (ia, fa) in a.variants.iter().enumerate() {
            if let Some((ib, _fb)) = b
                .variants
                .iter()
                .enumerate()
                .find(|(ib, fb)| !bok[*ib] && mergeable_variants(fa, fb, level))
            {
                aok[ia] = true;
                bok[ib] = true;
            } else {
                bail!("Variant in first enum has no match: {:?}", fa);
            }
        }
    }

    Ok(())
}

fn mergeable_variants(a: &EnumVariant, b: &EnumVariant, level: CheckLevel) -> bool {
    let mut res = true;
    if level >= CheckLevel::Layout {
        res &= a.value == b.value;
    }
    if level >= CheckLevel::Names {
        res &= a.name == b.name;
    }
    if level >= CheckLevel::Descriptions {
        res &= a.description == b.description;
    }
    res
}
