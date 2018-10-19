#![allow(dead_code)]
#![allow(non_camel_case_types)]

use core::ffi::c_void;

pub type BOOLEAN = bool;
pub type INTN = isize;
pub type UINTN = usize;
pub type INT8 = i8;
pub type UINT8 = u8;
pub type INT16 = i16;
pub type UINT16 = u16;
pub type INT32 = i32;
pub type UINT32 = u32;
pub type INT64 = i64;
pub type UINT64 = u64;
pub type INT128 = i128;
pub type UINT128 = u128;
pub type CHAR8 = u8;
pub type CHAR16 = u16;
pub type VOID = c_void;

pub type EFI_STATUS = UINTN;
pub type EFI_HANDLE = UINTN;
pub type EFI_EVENT = UINTN;
pub type EFI_LBA = UINT64;
pub type EFI_TPL = UINTN;

pub type EFI_ALLOCATE_TYPE = i32;
pub type EFI_MEMORY_TYPE = i32;
pub type EFI_PHYSICAL_ADDRESS = usize;

pub type EFI_SYSTEM_TABLE = VOID;
