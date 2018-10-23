use alloc::prelude::Vec;
use efi::{EfiMemoryDescriptor, EfiStatus, MemoryPtr, SystemTable, PGSIZE};
use gap_array::GapArray;

pub struct MemoryMap {
    pub key: usize,
    desc: GapArray<EfiMemoryDescriptor>,
}

#[derive(Clone, Copy)]
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

    pub fn find_segment(&self, query: MemoryQuery) -> Result<MemorySegment, EfiStatus> {
        let MemoryQuery::KthBiggest(k) = query;
        let mut top = Vec::with_capacity(k + 2);
        for desc in self.desc.iter() {
            top.push(desc);
            top.sort_unstable_by(|left, right| right.n_pages.cmp(&left.n_pages));
            if top.len() > k + 1 {
                top.truncate(k + 1);
            }
        }
        if top.len() > k {
            Ok(MemorySegment {
                start: top[k].vstart,
                size: (top[k].n_pages as usize) * PGSIZE,
            })
        } else {
            Err(EfiStatus::LoadError)
        }
    }
}
