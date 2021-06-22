use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
        for name in topological_sort(deps) {
            let block = ir.blocks.get(&name).unwrap();
            if let Some(parent_name) = &block.extends {
                let parent = ir.blocks.get(parent_name).unwrap();

                let items = parent.items.clone();
                let block = ir.blocks.get_mut(&name).unwrap();

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
        for name in topological_sort(deps) {
            let fieldset = ir.fieldsets.get(&name).unwrap();
            if let Some(parent_name) = &fieldset.extends {
                let parent = ir.fieldsets.get(parent_name).unwrap();

                let items = parent.fields.clone();
                let fieldset = ir.fieldsets.get_mut(&name).unwrap();

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

fn topological_sort(vals: HashMap<String, Option<String>>) -> Vec<String> {
    for (name, dep) in &vals {
        info!("{:?} => {:?}", name, dep);
    }

    let mut done = HashSet::new();
    let mut res = Vec::new();
    while done.len() != vals.len() {
        for (name, dep) in &vals {
            if done.contains(name) {
                continue;
            }
            if let Some(dep) = dep {
                if !done.contains(dep) {
                    continue;
                }
            }
            info!("doing {:?} ", name);
            done.insert(name.clone());
            res.push(name.clone());
        }
    }
    res
}
