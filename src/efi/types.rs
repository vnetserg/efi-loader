use core::convert::From;

use super::ctypes::{ EFI_STATUS, EFI_HANDLE };

pub type EfiHandle = EFI_HANDLE;

pub enum EfiStatus {
    EfiSuccess
}

impl From<EfiStatus> for EFI_STATUS {
    fn from(item: EfiStatus) -> EFI_STATUS {
        0
    }
}

