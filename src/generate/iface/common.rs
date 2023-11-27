use core::marker::PhantomData;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RW;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct R;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct W;

/// Interface to read and write register values
pub trait Interface<K, V> {
    type Error: core::fmt::Debug;
    fn read(&mut self, addr: K) -> Result<V, Self::Error>;
    fn write(&mut self, addr: K, value: V) -> Result<(), Self::Error>;
}

mod sealed {
    use super::*;
    pub trait Access {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
}

pub trait Access: sealed::Access + Copy {}
impl Access for R {}
impl Access for W {}
impl Access for RW {}

pub trait Read: Access {}
impl Read for RW {}
impl Read for R {}

pub trait Write: Access {}
impl Write for RW {}
impl Write for W {}

#[derive(PartialEq, Eq)]
pub struct Reg<'a, I, K, V, A> {
    iface: &'a mut I,
    addr: K,
    phantom: PhantomData<(V, A)>,
}

impl<'a, I, K: Copy, V, A: Access> Reg<'a, I, K, V, A> {
    #[inline(always)]
    pub fn new(iface: &'a mut I, addr: K) -> Self {
        Self {
            iface,
            addr,
            phantom: PhantomData,
        }
    }
}

impl<'a, I: Interface<K, V>, K: Copy, V, A: Read> Reg<'a, I, K, V, A> {
    #[inline(always)]
    pub fn read(&mut self) -> Result<V, I::Error> {
        self.iface.read(self.addr)
    }
}

impl<'a, I: Interface<K, V>, K: Copy, V, A: Write> Reg<'a, I, K, V, A> {
    #[inline(always)]
    pub fn write_value(&mut self, val: V) -> Result<(), I::Error> {
        self.iface.write(self.addr, val)
    }
}

impl<'a, I: Interface<K, V>, K: Copy, V: Copy + core::default::Default, A: Write>
    Reg<'a, I, K, V, A>
{
    #[inline(always)]
    pub fn write<R>(&mut self, f: impl FnOnce(&mut V) -> R) -> Result<R, I::Error> {
        let mut val = Default::default();
        let res = f(&mut val);
        self.write_value(val)?;
        Ok(res)
    }
}

impl<'a, I: Interface<K, V>, K: Copy, V, A: Read + Write> Reg<'a, I, K, V, A> {
    #[inline(always)]
    pub fn modify<R>(&mut self, f: impl FnOnce(&mut V) -> R) -> Result<R, I::Error> {
        let mut val = self.read()?;
        let res = f(&mut val);
        self.write_value(val)?;
        Ok(res)
    }
}
