use super::ctypes::EFI_SYSTEM_TABLE;
use super::{EfiAllocateType, EfiHandle, EfiMemoryType, EfiStatus, MemoryPtr};

pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

pub struct SystemTable {
    pub header: TableHeader,
    pub firmware_vendor: *const u16,
    pub console_in_handle: EfiHandle,
    pub con_in: &'static EfiSimpleTextInputProtocol,
    pub console_out_handle: EfiHandle,
    pub con_out: &'static EfiSimpleTextOutputProtocol,
    pub standard_error_handle: EfiHandle,
    pub con_err: &'static EfiSimpleTextOutputProtocol,
    pub runtime_services: &'static RuntimeServices,
    pub boot_services: &'static BootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: &'static ConfigurationTable,
}

impl SystemTable {
    pub unsafe fn claim(ptr: *const EFI_SYSTEM_TABLE) -> &'static SystemTable {
        core::mem::transmute::<*const EFI_SYSTEM_TABLE, &'static SystemTable>(ptr)
    }
}

pub struct BootServices {}

impl BootServices {
    pub fn allocate_pages(
        &self,
        _alloc_type: EfiAllocateType,
        _memory_type: EfiMemoryType,
        _n_pages: usize,
    ) -> Result<MemoryPtr, EfiStatus> {
        Ok(0)
    }

    pub fn get_memory_map(
        &self,
        _bufsize: &mut usize,
        _memptr: MemoryPtr,
        _map_key: &mut usize,
    ) -> Result<EfiMemoryDescriptorArray, EfiStatus> {
        Err(EfiStatus::LoadError)
    }

    pub fn free_pages(&self, _memptr: MemoryPtr, _n_pages: usize) -> Result<(), EfiStatus> {
        Ok(())
    }
}

pub struct EfiMemoryDescriptorArray {}

pub struct RuntimeServices {}
pub struct ConfigurationTable {}
pub struct EfiSimpleTextInputProtocol {}
pub struct EfiSimpleTextOutputProtocol {}
