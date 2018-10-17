use super::ctypes::{ EFI_STATUS, EFI_HANDLE };

pub type EfiHandle = EFI_HANDLE;

pub enum EfiStatus {
    EfiSuccess
}

impl EfiStatus {
    pub fn to_c(&self) -> EFI_STATUS {
        return 0;
    }
}

