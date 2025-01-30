

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::ir::*;

use super::common::{match_all, RegexSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResizeEnums {
    #[serde(rename = "enum")]
    emumm: RegexSet,
    bit_size: u32,
}

impl ResizeEnums {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let ids = match_all(ir.enums.keys().cloned(), &self.emumm);

        if self.bit_size == 0 {
            panic!("Cannot resize an enum to 0 bits (delete the enum?)");
        }

        // Resize the enums
        for enumm in ids.iter() {
            log::info!("Resizing enum {} to {} bits", enumm, self.bit_size);

            let enumm = ir.enums.get_mut(enumm).unwrap();
            enumm.bit_size = self.bit_size;
        }

        for enumm in ids.iter() {
            verify_variants(ir, enumm)?;
            update_uses(ir, enumm)?;
        }

        Ok(())
    }
}

/// Verify all enum variants fit within the bit size of the enum after resize.
fn verify_variants(ir: &IR, enumm: &str) -> anyhow::Result<()> {
    let e = ir.enums.get(enumm).unwrap();
    let max_value = 2_u64.checked_pow(e.bit_size)
        .with_context(|| format!("Bit size is too large"))?
        .checked_sub(1)
        .with_context(|| format!("New bit size is invalid: {}", e.bit_size))?;
    let mut error = false;

    for variant in e.variants.iter() {
        if variant.value > max_value {
            log::error!(
                "{}::{} (value: {}) is out of range as a result of resize to {} bits",
                enumm, variant.name, variant.value, e.bit_size
            );
            error |= true;
        }
    }

    if error {
        panic!();
    }

    Ok(())
}

fn update_uses(ir: &mut IR, enumm: &str) -> anyhow::Result<()> {
    let fieldsets = ir
        .fieldsets
        .iter()
        .filter(|(_, fs)| fs.fields.iter().any(|f| f.enumm.as_deref() == Some(enumm)))
        .map(|(name, _)| name)
        .cloned()
        .collect::<Vec<_>>();

    let bit_size = ir.enums.get(enumm).unwrap().bit_size;

    for fs_name in fieldsets {
        let fs = ir.fieldsets.get_mut(&fs_name).unwrap();

        for field in fs
            .fields
            .iter_mut()
            .filter(|f| f.enumm.as_deref() == Some(enumm))
        {
            field.bit_size = bit_size;
        }

        let mut error = false;

        // Verify there are no overlapping fields after resizing enums.
        for (i1, i2) in Pairs::new(fs.fields.iter()) {
            // expand every BitOffset to a Vec<RangeInclusive>,
            // and compare at that level
            'COMPARE: for i1_range in i1.bit_offset.clone().into_ranges(i1.bit_size) {
                for i2_range in i2.bit_offset.clone().into_ranges(i2.bit_size) {
                    if i2_range.end() > i1_range.start() && i1_range.end() > i2_range.start() {
                        log::error!(
                            "fieldset {}: fields overlap: {} {}",
                            fs_name, i1.name, i2.name
                        );
                        error |= true;
                        break 'COMPARE;
                    }
                }
            }
        }

        if error {
            panic!();
        }
    }

    Ok(())
}

struct Pairs<U: Iterator + Clone> {
    head: Option<U::Item>,
    tail: U,
    next: U,
}

impl<U: Iterator + Clone> Pairs<U> {
    fn new(mut iter: U) -> Self {
        let head = iter.next();
        Pairs {
            head,
            tail: iter.clone(),
            next: iter,
        }
    }
}

impl<U: Iterator + Clone> Iterator for Pairs<U>
where
    U::Item: Clone,
{
    type Item = (U::Item, U::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.head.as_ref()?.clone();

        if let Some(b) = self.tail.next() {
            return Some((a, b));
        }

        match self.next.next() {
            Some(new_head) => {
                self.head = Some(new_head);
                self.tail = self.next.clone();
                self.next()
            }
            None => None,
        }
    }
}
