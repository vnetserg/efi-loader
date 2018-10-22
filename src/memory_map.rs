use efi::{EfiMemoryDescriptor, EfiStatus, MemoryPtr, SystemTable};
use gapvec::GapVec;

pub struct MemoryMap {
    pub key: usize,
    desc: GapVec<EfiMemoryDescriptor>,
}

pub struct MemorySegment {
    pub start: MemoryPtr,
    pub size: usize,
}

pub enum MemoryQuery {
    KthBiggest(usize),
}

impl MemoryMap {
    pub fn current(table: &SystemTable) -> Result<MemoryMap, EfiStatus> {
        let mut byte_len = 0;
        unsafe {
            table
                .boot_services
                .get_memory_map(&mut byte_len, 0 as MemoryPtr)
                .expect_err("failed to get memory map length.");
        }

        let mut desc = GapVec::<EfiMemoryDescriptor>::with_byte_len(byte_len);
        let key = attempt!(
            unsafe {
                table
                    .boot_services
                    .get_memory_map(&mut byte_len, desc.as_ptr() as MemoryPtr)
            },
            "failed to get memory map."
        );

        Ok(MemoryMap { key, desc })
    }

    pub fn find_segment(&self, _query: MemoryQuery) -> Result<MemorySegment, EfiStatus> {
        Ok(MemorySegment { start: 0, size: 0 })
    }
}
