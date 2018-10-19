use super::ctypes::EFI_HANDLE;
use core::ops::Try;

pub type EfiHandle = EFI_HANDLE;
pub type MemoryPtr = usize;

#[allow(overflowing_literals)]
#[allow(dead_code)]
#[repr(C)]
pub enum EfiStatus {
    Success = 0,
    LoadError = 0x8000000000000001,
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
