use super::ctypes::{EFI_HANDLE, EFI_STATUS};
use core::convert::From;
use core::fmt;
use core::fmt::{Debug, Display};
use core::ops::Try;

pub type EfiHandle = EFI_HANDLE;
pub type MemoryPtr = usize;

#[allow(overflowing_literals)]
#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub enum EfiStatus {
    Success = 0,
    LoadError = 0x8000000000000001,
}

// Display is equivalent to Debug
impl Display for EfiStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<EFI_STATUS> for EfiStatus {
    fn from(status: EFI_STATUS) -> Self {
        if status == 0 {
            EfiStatus::Success
        } else {
            EfiStatus::LoadError
        }
    }
}

impl Try for EfiStatus {
    type Ok = EfiStatus;
    type Error = EfiStatus;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        if let EfiStatus::Success = self {
            Ok(EfiStatus::Success)
        } else {
            Err(self)
        }
    }

    fn from_error(v: Self::Error) -> Self {
        v
    }

    fn from_ok(v: Self::Ok) -> Self {
        v
    }
}

#[allow(dead_code)]
#[repr(C)]
pub enum EfiAllocateType {
    AnyPages,
    MaxAddress,
    Address,
    Max,
}

#[allow(dead_code)]
#[repr(C)]
pub enum EfiMemoryType {
    ReservedMemory,
    LoaderCode,
    LoaderData,
}

#[repr(C)]
pub struct EfiMemoryDescriptor {}
