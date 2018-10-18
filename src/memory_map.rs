use efi::{ EfiStatus, SystemTable, MemoryPtr, EfiAllocateType,
           EfiMemoryType, PGSIZE, EfiMemoryDescriptorArray };

pub struct MemoryMap {
    pub descarr: EfiMemoryDescriptorArray,
    pub map_key: usize,
}

pub struct MemorySegment {
    pub start: MemoryPtr,
    pub size: usize
}

pub enum MemoryQuery {
    KthBiggest(usize)
}

impl MemoryMap {
    pub fn current(table: &SystemTable) -> Result<MemoryMap, EfiStatus> {
        let mut n_pages = 1;
        loop {
            let memptr = attempt!(
                table.boot_services.allocate_pages(
                    EfiAllocateType::AnyPages,
                    EfiMemoryType::LoaderData,
                    n_pages
                ),
                b"failed to allocate pages.\n"
            );

            let mut map_key = 0;
            let mut bufsize = n_pages * PGSIZE;
            let memmap_res = table.boot_services.get_memory_map(
                                &mut bufsize, memptr, &mut map_key);

            // Check if our buffer was big enough
            if memmap_res.is_err() && bufsize > n_pages * PGSIZE {
                attempt!(table.boot_services.free_pages(memptr, n_pages),
                         b"failed to free pages.\n");
                n_pages = (bufsize / PGSIZE) + 1;
                continue;
            }

            let descarr = attempt!(memmap_res,
                                   b"failed to get memory map.\n");

            return Ok(MemoryMap{ descarr, map_key });
        }
    }

    pub fn find_segment(&self, _query: MemoryQuery)
            -> Result<MemorySegment, EfiStatus> {
        Ok(MemorySegment{ start: 0, size: 0})
    }
}
