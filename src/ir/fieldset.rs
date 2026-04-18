use serde::{Deserialize, Serialize};

use super::*;

fn sort_fn(a: &Field, b: &Field) -> std::cmp::Ordering {
    a.bit_offset.cmp(&b.bit_offset).then(a.name.cmp(&b.name))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnorderedFieldSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extends: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "default_32", skip_serializing_if = "is_32")]
    pub bit_size: u32,

    pub fields: Vec<Field>,
}

impl From<UnorderedFieldSet> for FieldSet {
    fn from(mut value: UnorderedFieldSet) -> Self {
        value.fields.sort_by(sort_fn);
        let fields = value.fields.into_iter().map(FieldView).collect();

        Self {
            extends: value.extends,
            description: value.description,
            bit_size: value.bit_size,
            fields,
        }
    }
}

impl From<FieldSet> for UnorderedFieldSet {
    fn from(value: FieldSet) -> Self {
        Self {
            extends: value.extends,
            description: value.description,
            bit_size: value.bit_size,
            fields: value.fields.into_iter().map(|v| v.0).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "UnorderedFieldSet", into = "UnorderedFieldSet")]
pub struct FieldSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extends: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "default_32", skip_serializing_if = "is_32")]
    pub bit_size: u32,

    /// The fields in this field set, always ordered according
    /// to `sort_fn`
    fields: Vec<FieldView>,
}

impl FieldSet {
    pub fn take_fields(&mut self) -> Vec<Field> {
        let fields = std::mem::take(&mut self.fields);
        fields.into_iter().map(Into::into).collect()
    }

    /// Iterate over the fields of this fieldset.
    ///
    /// They are ordered by offset and name.
    pub fn fields(&self) -> impl ExactSizeIterator<Item = &FieldView> + Clone {
        self.fields.iter()
    }

    /// Mutably iterate over the fields of this fieldset.
    ///
    /// They are ordered by offset and name.
    pub fn fields_mut(&mut self) -> impl ExactSizeIterator<Item = &mut FieldView> {
        self.fields.iter_mut()
    }

    pub fn retain_fields<F>(&mut self, f: F)
    where
        F: FnMut(&FieldView) -> bool,
    {
        self.fields.retain(f);
    }

    pub fn push(&mut self, field: Field) {
        let index = self.fields.iter().enumerate().find_map(|(idx, f)| {
            if sort_fn(f.as_ref(), &field).is_gt() {
                Some(idx)
            } else {
                None
            }
        });

        if let Some(index) = index {
            self.fields.insert(index, field.into())
        } else {
            self.fields.push(field.into());
        }
    }

    pub fn extend(&mut self, mut fields: Vec<Field>) {
        // Sort provided list
        fields.sort_by(sort_fn);

        let mut result = Vec::with_capacity(self.fields.len() + fields.len());

        // Merge-sort
        let mut existing_fields = std::mem::take(&mut self.fields).into_iter();
        let mut new_fields = fields.into_iter().map(FieldView::from);

        let mut existing = None;
        let mut new = None;
        loop {
            existing = existing.or_else(|| existing_fields.next());
            new = new.or_else(|| new_fields.next());

            let (new_e, new_n) = match (existing, new) {
                (None, None) => break,
                (None, Some(v)) | (Some(v), None) => {
                    result.push(v);
                    (None, None)
                }
                (Some(a), Some(b)) => match sort_fn(a.as_ref(), b.as_ref()) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                        result.push(a);
                        (None, Some(b))
                    }
                    std::cmp::Ordering::Greater => {
                        result.push(b);
                        (Some(a), None)
                    }
                },
            };

            existing = new_e;
            new = new_n;
        }

        self.fields = result;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldView(Field);

impl From<Field> for FieldView {
    fn from(value: Field) -> Self {
        Self(value)
    }
}

impl From<FieldView> for Field {
    fn from(value: FieldView) -> Self {
        value.0
    }
}

impl FieldView {
    // Bit offset is immutable for ordering purposes
    pub fn bit_offset(&self) -> &BitOffset {
        &self.0.bit_offset
    }

    // Name is immutable for ordering purposes
    pub fn name(&self) -> &String {
        &self.0.name
    }

    pub fn description(&self) -> Option<&String> {
        self.0.description.as_ref()
    }

    pub fn description_opt_mut(&mut self) -> &mut Option<String> {
        &mut self.0.description
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.0.description = description;
    }

    pub fn bit_size(&self) -> u32 {
        self.0.bit_size
    }

    pub fn set_bit_size(&mut self, bit_size: u32) {
        self.0.bit_size = bit_size;
    }

    pub fn array(&self) -> Option<&Array> {
        self.0.array.as_ref()
    }

    pub fn enumm(&self) -> Option<&String> {
        self.0.enumm.as_ref()
    }

    pub fn enumm_opt_mut(&mut self) -> &mut Option<String> {
        &mut self.0.enumm
    }

    pub fn set_enumm(&mut self, enumm: Option<String>) {
        self.0.enumm = enumm;
    }
}

impl AsRef<Field> for FieldView {
    fn as_ref(&self) -> &Field {
        &self.0
    }
}
