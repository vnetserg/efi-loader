#![macro_use]

macro_rules! efi_main {
    ($(fn main($handle:ident: $htype:ty, $table:ident: $stype:ty)
            -> $ret:ty $body:block)+) => ($(

        fn main($handle: $htype, $table: $stype) -> $ret
            $body
                
        #[no_mangle]
        pub extern "stdcall" fn efi_main(handle: efi::ctypes::EFI_HANDLE,
                                         st: *const efi::ctypes::EFI_SYSTEM_TABLE)
                -> efi::ctypes::EFI_STATUS {
            unsafe {
                efi::ffi::InitializeLib(handle, st);
            }
            let table = SystemTable::new(st);
            let status = main(handle, table);
            return status.to_c();
        }
    )+);

}
