use core::ops::Try;
use core::convert::From;

use super::ctypes::{ EFI_STATUS, EFI_HANDLE };

pub type EfiHandle = EFI_HANDLE;

pub enum EfiStatus {
    EfiSuccess = 0,
    EfiLoadError = 0x8000000000000001
}


impl Try for EfiStatus {
    type Ok = EfiStatus;
    type Error = EfiStatus;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        if let EfiStatus::EfiSuccess = self {
            Ok(EfiStatus::EfiSuccess)
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

