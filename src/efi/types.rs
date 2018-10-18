use core::ops::Try;
use super::ctypes::EFI_HANDLE;

pub type EfiHandle = EFI_HANDLE;
pub type MemoryPtr = usize;

#[allow(overflowing_literals)]
#[allow(dead_code)]
pub enum EfiStatus {
    Success = 0,
    LoadError = 0x8000000000000001
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


pub enum EfiAllocateType {
    AnyPages,
}


pub enum EfiMemoryType {
    LoaderData,
}
