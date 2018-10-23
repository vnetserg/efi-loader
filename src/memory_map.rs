use efi::{EfiMemoryDescriptor, EfiStatus, MemoryPtr, SystemTable};
use gap_array::GapArray;

pub struct MemoryMap {
    pub key: usize,
    desc: GapArray<EfiMemoryDescriptor>,
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
        let mut desc_size = 0;
        unsafe {
            table
                .boot_services
                .get_memory_map(&mut byte_len, 0 as MemoryPtr, &mut desc_size)
                .expect_err("failed to get memory map length.");
        }

        let mut desc = GapArray::<EfiMemoryDescriptor>::with_byte_len(byte_len);
        let key = attempt!(
            unsafe {
                table.boot_services.get_memory_map(
                    &mut byte_len,
                    desc.as_mut_ptr() as MemoryPtr,
                    &mut desc_size,
                )
            },
            "failed to get memory map."
        );

        unsafe {
            desc.set_layout(desc_size, byte_len / desc_size);
        }

        Ok(MemoryMap { key, desc })
    }

    pub fn find_segment(&self, _query: MemoryQuery) -> Result<MemorySegment, EfiStatus> {
        Ok(MemorySegment { start: 0, size: 0 })
    }
}
