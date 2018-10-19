use super::ctypes::*;

extern "C" {
    pub fn Print(s: *const CHAR16, ...) -> ();
    pub fn InitializeLib(handle: EFI_HANDLE, table: *const EFI_SYSTEM_TABLE) -> ();
}
