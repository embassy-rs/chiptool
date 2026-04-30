use core::marker::PhantomData;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RW;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct R;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct W;

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

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Reg<T: Copy, A: Access> {
    ptr: *mut u8,
    phantom: PhantomData<*mut (T, A)>,
}
unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}

impl<T: Copy, A: Access> Reg<T, A> {
    #[allow(clippy::missing_safety_doc)]
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self {
            ptr: ptr as _,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr as _
    }
}

impl<T: Copy, A: Read> Reg<T, A> {
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { (self.ptr as *mut T).read_volatile() }
    }
}

impl<T: Copy, A: Write> Reg<T, A> {
    #[inline(always)]
    pub fn write_value(&self, val: T) {
        unsafe { (self.ptr as *mut T).write_volatile(val) }
    }
}

impl<T: Default + Copy, A: Write> Reg<T, A> {
    #[inline(always)]
    pub fn write(&self, f: impl FnOnce(&mut T)) {
        let mut val = Default::default();
        f(&mut val);
        self.write_value(val);
    }
}

impl<T: Copy, A: Read + Write> Reg<T, A> {
    #[inline(always)]
    pub fn modify(&self, f: impl FnOnce(&mut T)) {
        let mut val = self.read();
        f(&mut val);
        self.write_value(val);
    }
}

pub trait FromPtr: Copy {
    unsafe fn from_ptr(ptr: *mut u8) -> Self;
}

impl<T: Copy, A: Access> FromPtr for Reg<T, A> {
    unsafe fn from_ptr(ptr: *mut u8) -> Self {
        unsafe { Reg::<T, A>::from_ptr(ptr as *mut T) }
    }
}

pub struct Array<T> {
    ptr: *mut u8,
    stride: usize,
    len: usize,
    _type: PhantomData<T>,
}

impl<T: Clone> Clone for Array<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            stride: self.stride,
            len: self.len,
            _type: self._type,
        }
    }
}
impl<T: Copy> Copy for Array<T> {}

impl<T> Array<T> {
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }
}

impl<T: FromPtr> Array<T> {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8, stride: usize, len: usize) -> Self {
        Self {
            ptr,
            stride,
            len,
            _type: PhantomData,
        }
    }

    #[inline(always)]
    pub fn get(&self, n: usize) -> T {
        assert!(n < self.len());
        unsafe { T::from_ptr(self.ptr.wrapping_add(n * self.stride)) }
    }
}

impl<T: FromPtr> Iterator for Array<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        self.ptr = self.ptr.wrapping_add(self.stride);
        self.len -= 1;

        Some(self.get(0))
    }
}

pub struct CursedArray<T> {
    ptr: *mut u8,
    offsets: &'static [usize],
    _type: PhantomData<T>,
}

impl<T: Clone> Clone for CursedArray<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            offsets: self.offsets,
            _type: self._type,
        }
    }
}
impl<T: Copy> Copy for CursedArray<T> {}

impl<T> CursedArray<T> {
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.offsets.len()
    }
}

impl<T: FromPtr> CursedArray<T> {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut u8, offsets: &'static [usize]) -> Self {
        Self {
            ptr,
            offsets,
            _type: PhantomData,
        }
    }

    #[inline(always)]
    pub fn get(&self, n: usize) -> T {
        assert!(n < self.len());
        unsafe { T::from_ptr(self.ptr.wrapping_add(self.offsets[n])) }
    }
}

impl<T: FromPtr> Iterator for CursedArray<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None;
        }

        self.offsets = &self.offsets[1..];
        Some(self.get(0))
    }
}
