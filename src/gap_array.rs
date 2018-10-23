use alloc::prelude::{Box, Vec};
use core::marker::PhantomData;
use core::mem::{size_of, transmute};
use core::ops::Index;

pub struct GapArray<T> {
    data: Box<[u8]>,
    size: usize,
    count: usize,
    value_type: PhantomData<T>,
}

impl<T> GapArray<T> {
    pub fn with_byte_len(len: usize) -> GapArray<T> {
        let mut data = Vec::with_capacity(len);
        unsafe {
            data.set_len(len);
        }
        GapArray {
            data: data.into_boxed_slice(),
            value_type: PhantomData,
            size: size_of::<T>(),
            count: 0,
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        unsafe { transmute::<*mut u8, *mut T>(self.data.as_mut_ptr()) }
    }

    pub unsafe fn set_layout(&mut self, size: usize, count: usize) {
        self.size = size;
        self.count = count;
    }
}

impl<T> Index<usize> for GapArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let start = self.size * index;
        if index >= self.count
            || start >= self.data.len()
            || start + size_of::<T>() > self.data.len()
        {
            panic!("GapArray index out of bounds");
        }
        unsafe { transmute::<&u8, &T>(&self.data[index]) }
    }
}

/*
impl <T> IntoIterator for GapArray<T> {

}
*/
