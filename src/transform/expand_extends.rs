use anyhow::{bail, Result};
use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandExtends {}

impl ExpandExtends {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        // Expand blocks
        let deps = ir
            .blocks
            .iter()
            .map(|(k, v)| (k.clone(), v.extends.clone()))
            .collect();
        for name in topological_sort(deps)? {
            let block = get_ref!(ir, blocks, &name)?;
            if let Some(parent_name) = &block.extends {
                let parent = get_ref!(ir, blocks, parent_name)?;

                let items = parent.items.clone();
                let block = get_mut!(ir, blocks, &name)?;

                for i in items {
                    if !block.items.iter().any(|j| j.name == i.name) {
                        block.items.push(i);
                    }
                }
            }
        }
        // Expand fiedsets
        let deps = ir
            .fieldsets
            .iter()
            .map(|(k, v)| (k.clone(), v.extends.clone()))
            .collect();
        for name in topological_sort(deps)? {
            let fieldset = get_ref!(ir, fieldsets, &name)?;
            if let Some(parent_name) = &fieldset.extends {
                let parent = get_ref!(ir, fieldsets, parent_name)?;

                let items = parent.fields.clone();
                let fieldset = get_mut!(ir, fieldsets, &name)?;

                for i in items {
                    if !fieldset.fields.iter().any(|j| j.name == i.name) {
                        fieldset.fields.push(i);
                    }
                }
            }
        }

        Ok(())
    }
}

fn topological_sort(vals: BTreeMap<String, Option<String>>) -> Result<Vec<String>> {
    for (name, dep) in &vals {
        info!("{:?} → {:?}", name, dep);
    }

    let mut done = BTreeSet::new();
    let mut res = Vec::new();

    while done.len() != vals.len() {
        for (name, dep) in &vals {
            if done.contains(name) {
                continue;
            }
            if let Some(dep) = dep {
                if !vals.contains_key(dep) {
                    bail!("Couldn't resolve dependency for {name} → {dep}");
                }
                if !done.contains(dep) {
                    continue;
                }
            }
            info!("doing {:?} ", name);
            done.insert(name.clone());
            res.push(name.clone());
        }
    }

    Ok(res)
}
