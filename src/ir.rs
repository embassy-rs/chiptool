use std::fmt;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::{collections::HashMap, num::NonZeroU32, rc::Rc};
use svd_parser::svd;

pub struct Id<T> {
    id: u32,
    phantom: PhantomData<T>,
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tn = std::any::type_name::<T>();
        let tn = tn.rsplit(':').next().unwrap();
        write!(f, "{}#{}", tn, self.id)
    }
}

impl<T> PartialEq<Self> for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.id)
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            phantom: PhantomData,
        }
    }
}

impl<T> Id<T> {
    fn new(id: u32) -> Self {
        Self {
            id,
            phantom: PhantomData,
        }
    }
}

pub struct Set<T> {
    map: HashMap<Id<T>, T>,
    next_id: u32,
}

impl<T: fmt::Debug> fmt::Debug for Set<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.map, f)
    }
}

impl<T> Set<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn put(&mut self, val: T) -> Id<T> {
        let id = Id::new(self.next_id);
        self.map.insert(id, val);
        self.next_id = self.next_id + 1;
        id
    }

    pub fn remove(&mut self, id: Id<T>) {
        self.map
            .remove(&id)
            .expect("removed an ID that's not present");
    }

    pub fn get(&self, id: Id<T>) -> &T {
        self.map.get(&id).unwrap()
    }
    pub fn get_mut(&mut self, id: Id<T>) -> &mut T {
        self.map.get_mut(&id).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Id<T>, &T)> {
        self.map.iter().map(|(id, val)| (*id, val))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Id<T>, &mut T)> {
        self.map.iter_mut().map(|(id, val)| (*id, val))
    }

    pub fn find(&self, mut f: impl FnMut(&T) -> bool) -> Option<(Id<T>, &T)> {
        for (&k, v) in self.map.iter() {
            if f(v) {
                return Some((k, v));
            }
        }
        None
    }

    pub fn find_mut(&mut self, mut f: impl FnMut(&T) -> bool) -> Option<(Id<T>, &mut T)> {
        for (&k, v) in self.map.iter_mut() {
            if f(v) {
                return Some((k, v));
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub cpu: Option<svd::Cpu>,
    pub interrupts: Vec<Interrupt>,
    pub instances: Set<PeripheralInstance>,
    pub peripherals: Set<Peripheral>,
    pub blocks: Set<Block>,
    pub fieldsets: Set<FieldSet>,
    pub enums: Set<Enum>,
}

impl Device {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            cpu: None,
            interrupts: Vec::new(),
            instances: Set::new(),
            peripherals: Set::new(),
            blocks: Set::new(),
            fieldsets: Set::new(),
            enums: Set::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Interrupt {
    pub name: String,
    pub description: Option<String>,
    pub value: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PeripheralInstance {
    pub path: Path,
    pub description: Option<String>,
    pub base_address: u32,

    pub peripheral: Id<Peripheral>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Peripheral {
    pub path: Path,
    pub block: Id<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub path: Path,
    pub items: Vec<BlockItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockItem {
    pub name: String,
    pub description: Option<String>,

    pub array: Option<Array>,
    pub byte_offset: u32,
    pub inner: BlockItemInner,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockItemInner {
    Register(Register),
    Block(Id<Block>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Array {
    pub len: u32,
    pub byte_stride: u32,
    //pub index_names: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Register {
    pub reset_value: Option<u64>,
    pub access: Access,
    pub bit_size: u32,
    pub fieldset: Option<Id<FieldSet>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Access {
    ReadWrite,
    Read,
    Write,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldSet {
    pub path: Path,
    pub description: Option<String>,

    pub bit_size: u32,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,

    pub bit_offset: u32,
    pub bit_size: u32,
    pub enumm: Option<Id<Enum>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {
    pub path: Path,
    pub description: Option<String>,

    pub bit_size: u32,
    pub variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub description: Option<String>,

    pub value: u64,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Path {
    pub modules: Vec<String>,
    pub name: String,
}

impl Path {
    pub fn new(modules: Vec<String>, name: String) -> Self {
        Self { modules, name }
    }

    pub fn new_from_string(s: &str) -> Self {
        let mut modules: Vec<String> = s.split("::").map(|x| x.to_string()).collect();
        let name = modules.pop().unwrap();
        Self { modules, name }
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();
        for x in &self.modules {
            write!(&mut res, "{}::", x).unwrap();
        }
        write!(&mut res, "{}", &self.name).unwrap();
        res
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in &self.modules {
            f.write_str(x)?;
            f.write_str("::")?;
        }
        f.write_str(&self.name)?;
        Ok(())
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Path(")?;
        for x in &self.modules {
            f.write_str(x)?;
            f.write_str("::")?;
        }
        f.write_str(&self.name)?;
        f.write_str(")")?;
        Ok(())
    }
}

pub trait Pathed {
    fn path(&self) -> &Path;
    fn path_mut(&mut self) -> &mut Path;
}
macro_rules! impl_pathed {
    ($self:ty ) => {
        impl Pathed for $self {
            fn path(&self) -> &Path {
                &self.path
            }
            fn path_mut(&mut self) -> &mut Path {
                &mut self.path
            }
        }
    };
}

impl_pathed!(Enum);
impl_pathed!(FieldSet);
impl_pathed!(Block);
impl_pathed!(Peripheral);
impl_pathed!(PeripheralInstance);
