use super::ctypes::*;
use super::{EfiAllocateType, EfiHandle, EfiMemoryDescriptor, EfiMemoryType, EfiStatus, MemoryPtr};

const SYSTEM_TABLE_SIGNATURE: u64 = 0x5453595320494249;
const BOOT_SERVICES_SIGNATURE: u64 = 0x56524553544f4f42;

#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct SystemTable {
    pub header: TableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    pub con_in: &'static EfiSimpleTextInputProtocol,
    pub console_out_handle: EfiHandle,
    pub con_out: &'static EfiSimpleTextOutputProtocol,
    pub standard_error_handle: EfiHandle,
    pub std_err: &'static EfiSimpleTextOutputProtocol,
    pub runtime_services: &'static RuntimeServices,
    pub boot_services: &'static BootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: &'static ConfigurationTable,
}

impl SystemTable {
    pub unsafe fn claim(ptr: *const EFI_SYSTEM_TABLE) -> &'static SystemTable {
        let table = core::mem::transmute::<*const EFI_SYSTEM_TABLE, &'static SystemTable>(ptr);
        if table.header.signature != SYSTEM_TABLE_SIGNATURE {
            panic!("System table signature mismatch.\n");
        }
        if table.boot_services.header.signature != BOOT_SERVICES_SIGNATURE {
            panic!("Boot services signature mismatch.\n");
        }
        table
    }
}

#[allow(dead_code)]
#[repr(C)]
pub struct BootServices {
    pub header: TableHeader,
    c_raise_tpl: extern "win64" fn(),
    c_restore_tpl: extern "win64" fn(),
    c_allocate_pages:
        extern "win64" fn(EFI_ALLOCATE_TYPE, EFI_MEMORY_TYPE, UINTN, *mut EFI_PHYSICAL_ADDRESS)
            -> EFI_STATUS,
    c_free_pages: extern "win64" fn(EFI_PHYSICAL_ADDRESS, UINTN) -> EFI_STATUS,
    c_get_memory_map: extern "win64" fn(
        *mut UINTN,
        *mut EfiMemoryDescriptor,
        *mut UINTN,
        *mut UINTN,
        *mut UINT32,
    ) -> EFI_STATUS,
}

impl BootServices {
    pub fn allocate_pages(
        &self,
        alloc_type: EfiAllocateType,
        memory_type: EfiMemoryType,
        n_pages: usize,
    ) -> Result<MemoryPtr, EfiStatus> {
        let mut addr: EFI_PHYSICAL_ADDRESS = 0;
        let c_status = (self.c_allocate_pages)(
            alloc_type as EFI_ALLOCATE_TYPE,
            memory_type as EFI_MEMORY_TYPE,
            n_pages as UINTN,
            &mut addr as *mut EFI_PHYSICAL_ADDRESS,
        );
        let status = EfiStatus::from(c_status);
        if let EfiStatus::Success = status {
            return Ok(addr as MemoryPtr);
        }
        return Err(status);
    }

    pub unsafe fn free_pages(&self, memptr: MemoryPtr, n_pages: usize) -> EfiStatus {
        let c_status = (self.c_free_pages)(memptr as EFI_PHYSICAL_ADDRESS, n_pages as UINTN);
        return EfiStatus::from(c_status);
    }

    pub unsafe fn get_memory_map(
        &self,
        bufsize: &mut usize,
        memptr: MemoryPtr,
        desc_size: &mut usize,
    ) -> Result<usize, EfiStatus> {
        let mut map_key = 0usize;
        let mut desc_version = 0u32;
        let c_status = (self.c_get_memory_map)(
            bufsize as *mut UINTN,
            memptr as *mut EfiMemoryDescriptor,
            &mut map_key as *mut UINTN,
            desc_size as *mut UINTN,
            &mut desc_version as *mut UINT32,
        );
        let status = EfiStatus::from(c_status);
        if let EfiStatus::Success = status {
            return Ok(map_key);
        }
        return Err(status);
    }
}

pub struct RuntimeServices {}
pub struct ConfigurationTable {}
pub struct EfiSimpleTextInputProtocol {}
pub struct EfiSimpleTextOutputProtocol {}
