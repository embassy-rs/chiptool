use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeCluster {
    pub block: RegexSet,
    pub clusters: Vec<ClusterDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterDef {
    pub name: String,
    pub subclusters: Vec<SubclusterDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubclusterDef {
    pub name: String,
    pub registers: Vec<RegisterMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterMapping {
    pub name: String,
    pub from: String,
}

impl MakeCluster {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.blocks.keys().cloned(), &self.block) {
            let block = ir.blocks.get(&id).unwrap().clone();

            // Check if any of the clusters already exist (skip chips that already have them)
            let has_existing = self.clusters.iter().any(|cluster| {
                block.items.iter().any(|i| i.name == cluster.name)
            });

            if has_existing {
                info!("Clusters already exist in {}, skipping", id);
                continue;
            }

            // Check if this block has any registers we're trying to remap
            let has_registers = self.clusters.iter().any(|cluster| {
                cluster.subclusters.iter().any(|subcluster| {
                    subcluster.registers.iter().any(|reg| {
                        block.items.iter().any(|i| i.name == reg.from)
                    })
                })
            });

            if !has_registers {
                info!("Block {} doesn't have expected registers, skipping", id);
                continue;
            }

            // Create peripheral-specific block names
            let safe_id = id.replace("::", "_");

            self.create_clusters_for_block(ir, &block, &id, &safe_id)?;
        }

        Ok(())
    }

    fn create_clusters_for_block(
        &self,
        ir: &mut IR,
        block: &Block,
        block_id: &str,
        safe_id: &str,
    ) -> anyhow::Result<()> {
        for cluster_def in &self.clusters {
            self.create_single_cluster(ir, block, block_id, safe_id, cluster_def)?;
        }
        Ok(())
    }

    fn create_single_cluster(
        &self,
        ir: &mut IR,
        block: &Block,
        block_id: &str,
        safe_id: &str,
        cluster_def: &ClusterDef,
    ) -> anyhow::Result<()> {
        // Collect registers for each subcluster
        let mut subcluster_data = Vec::new();
        for subcluster_def in &cluster_def.subclusters {
            let (regs, base) = self.collect_subcluster_registers(block, &subcluster_def.registers);
            if !regs.is_empty() {
                subcluster_data.push((subcluster_def, regs, base));
            }
        }

        if subcluster_data.is_empty() {
            return Ok(());
        }

        // Find the minimum offset to use as the cluster base
        let cluster_base = subcluster_data.iter().map(|(_, _, base)| *base).min().unwrap();

        // Create subcluster blocks
        for (subcluster_def, regs, base) in &subcluster_data {
            let subcluster_block_name = format!("{}_{}", safe_id, cluster_def.name);
            let subcluster_block_name = format!("{}_{}", subcluster_block_name, subcluster_def.name);

            ir.blocks.insert(subcluster_block_name.clone(), Block {
                extends: None,
                description: Some(format!("{} {} subcluster", cluster_def.name, subcluster_def.name)),
                items: regs.iter().map(|(name, offset, access)| BlockItem {
                    name: name.clone(),
                    description: None,
                    array: None,
                    byte_offset: offset - base,
                    inner: BlockItemInner::Register(Register {
                        access: access.clone(),
                        bit_size: 32,
                        fieldset: None,
                    }),
                }).collect(),
            });
        }

        // Create main cluster block
        let cluster_block_name = format!("{}_{}", safe_id, cluster_def.name);
        let cluster_items: Vec<BlockItem> = subcluster_data.iter().map(|(subcluster_def, _, base)| {
            let subcluster_block_name = format!("{}_{}", cluster_block_name, subcluster_def.name);
            BlockItem {
                name: subcluster_def.name.clone(),
                description: None,
                array: None,
                byte_offset: base - cluster_base,
                inner: BlockItemInner::Block(BlockItemBlock { block: subcluster_block_name }),
            }
        }).collect();

        ir.blocks.insert(cluster_block_name.clone(), Block {
            extends: None,
            description: Some(format!("{} cluster", cluster_def.name)),
            items: cluster_items,
        });

        // Add cluster to peripheral block and remove original registers
        let b = ir.blocks.get_mut(block_id).unwrap();

        // Only remove registers that were actually found and added to the cluster
        let registers_to_remove: Vec<String> = cluster_def.subclusters.iter()
            .flat_map(|subcluster| &subcluster.registers)
            .filter(|reg| block.items.iter().any(|item| item.name == reg.from))
            .map(|reg| reg.from.clone())
            .collect();

        let original_count = b.items.len();
        b.items.retain(|item| !registers_to_remove.contains(&item.name));
        let removed = original_count - b.items.len();

        b.items.push(BlockItem {
            name: cluster_def.name.clone(),
            description: None,
            array: None,
            byte_offset: cluster_base,
            inner: BlockItemInner::Block(BlockItemBlock { block: cluster_block_name }),
        });

        info!("Added {} cluster to {} (removed {} registers)", cluster_def.name, block_id, removed);

        Ok(())
    }

    fn collect_subcluster_registers(
        &self,
        block: &Block,
        mappings: &[RegisterMapping],
    ) -> (Vec<(String, u32, Access)>, u32) {
        let mut regs = Vec::new();
        let mut min_offset = u32::MAX;

        for mapping in mappings {
            if let Some(item) = block.items.iter().find(|i| i.name == mapping.from) {
                let access = match &item.inner {
                    BlockItemInner::Register(reg) => reg.access.clone(),
                    _ => Access::ReadWrite,
                };
                regs.push((mapping.name.clone(), item.byte_offset, access));
                min_offset = min_offset.min(item.byte_offset);
            }
        }

        (regs, if min_offset == u32::MAX { 0 } else { min_offset })
    }
}
