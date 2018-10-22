use alloc::vec::Vec;

pub struct GapVec<T> {
    data: Vec<T>,
}

impl<T> GapVec<T> {
    pub fn with_byte_len(_len: usize) -> GapVec<T> {
        GapVec::<T> { data: Vec::new() }
    }

    pub fn as_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }
}
