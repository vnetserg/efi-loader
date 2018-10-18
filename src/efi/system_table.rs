use super::{ EfiAllocateType, EfiMemoryType, EfiStatus, MemoryPtr };
use super::ctypes::EFI_SYSTEM_TABLE;


pub struct SystemTable {
    pub boot_services: BootServices,
}

impl SystemTable {
    pub fn new(_ptr: *const EFI_SYSTEM_TABLE) -> SystemTable {
        return SystemTable{ boot_services: BootServices{} }
    }
}


pub struct BootServices {}

impl BootServices {
    pub fn allocate_pages(&self,
                          _alloc_type: EfiAllocateType,
                          _memory_type: EfiMemoryType,
                          _n_pages: usize)
            -> Result<MemoryPtr, EfiStatus> {
        Ok(0)
    }

    pub fn get_memory_map(&self, _bufsize: &mut usize, _memptr: MemoryPtr,
                          _map_key: &mut usize)
            -> Result<EfiMemoryDescriptorArray, EfiStatus> {
        Err(EfiStatus::LoadError)
    }

    pub fn free_pages(&self, _memptr: MemoryPtr, _n_pages: usize)
            -> Result<(), EfiStatus> {
        Ok(())
    }
}


pub struct EfiMemoryDescriptorArray {}

