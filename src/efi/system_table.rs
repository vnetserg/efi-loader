use super::ctypes::EFI_SYSTEM_TABLE;

pub struct SystemTable {}

impl SystemTable {
    pub fn new(_ptr: *const EFI_SYSTEM_TABLE) -> SystemTable {
        return SystemTable{}
    }
}
