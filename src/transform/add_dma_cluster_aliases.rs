use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddDmaClusterAliases {
    pub block: RegexSet,
    pub clusters: Vec<DmaCluster>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmaCluster {
    pub name: String,
    pub byte_offset: u32,
    pub subclusters: Vec<DmaSubcluster>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmaSubcluster {
    pub name: String,
    pub byte_offset: u32,
    pub registers: Vec<DmaRegisterAlias>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmaRegisterAlias {
    pub name: String,
    pub byte_offset: u32,
    pub target_register: String,
}

impl AddDmaClusterAliases {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for cluster_def in &self.clusters {
            // Create subcluster blocks
            for subcluster in &cluster_def.subclusters {
                let block_id = format!("{}_{}", cluster_def.name, subcluster.name);

                // Find the target registers to determine their properties
                let mut items = Vec::new();

                for reg_alias in &subcluster.registers {
                    // We'll add registers that point to the same offset as the target
                    // The actual register properties will be copied from target in the next step
                    items.push(BlockItem {
                        name: reg_alias.name.clone(),
                        description: None,
                        array: None,
                        byte_offset: reg_alias.byte_offset,
                        inner: BlockItemInner::Register(Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        }),
                    });
                }

                let block = Block {
                    extends: None,
                    description: Some(format!("DMA {} peripheral registers", subcluster.name)),
                    items,
                };

                ir.blocks.insert(block_id, block);
            }

            // Create main cluster block
            let mut cluster_items = Vec::new();
            for subcluster in &cluster_def.subclusters {
                let block_id = format!("{}_{}", cluster_def.name, subcluster.name);
                cluster_items.push(BlockItem {
                    name: subcluster.name.clone(),
                    description: None,
                    array: None,
                    byte_offset: subcluster.byte_offset,
                    inner: BlockItemInner::Block(BlockItemBlock { block: block_id }),
                });
            }

            let cluster_block = Block {
                extends: None,
                description: Some(format!("DMA {} peripheral clusters", cluster_def.name)),
                items: cluster_items,
            };

            ir.blocks.insert(cluster_def.name.clone(), cluster_block);
        }

        // Now add cluster references to matching peripheral blocks and remove original registers
        for id in match_all(ir.blocks.keys().cloned(), &self.block) {
            for cluster_def in &self.clusters {
                let b = ir.blocks.get_mut(&id).unwrap();

                // Check if cluster already exists
                if b.items.iter().any(|i| i.name == cluster_def.name) {
                    info!("Cluster {} already exists in {}, skipping", cluster_def.name, id);
                    continue;
                }

                // Collect all target register names that should be removed
                let mut registers_to_remove = Vec::new();
                for subcluster in &cluster_def.subclusters {
                    for reg_alias in &subcluster.registers {
                        registers_to_remove.push(reg_alias.target_register.clone());
                    }
                }

                // Remove the original registers since they're now accessible via cluster
                let original_count = b.items.len();
                b.items.retain(|item| !registers_to_remove.contains(&item.name));
                let removed_count = original_count - b.items.len();
                if removed_count > 0 {
                    info!("Removed {} original registers from block {}", removed_count, id);
                }

                // Add cluster reference
                b.items.push(BlockItem {
                    name: cluster_def.name.clone(),
                    description: None,
                    array: None,
                    byte_offset: cluster_def.byte_offset,
                    inner: BlockItemInner::Block(BlockItemBlock {
                        block: cluster_def.name.clone(),
                    }),
                });

                info!("Added cluster {} to block {}", cluster_def.name, id);
            }
        }

        Ok(())
    }
}
