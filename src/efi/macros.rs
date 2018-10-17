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
            let table = efi::SystemTable::new(st);
            let status = main(handle, table);
            return efi::ctypes::EFI_STATUS::from(status);
        }
    )+);

}

macro_rules! print {
    ( $s:literal $(, $x:expr )* ) => {
        {
            let mut __buf: [u16; $s.len()+1]
                = unsafe { core::mem::uninitialized() };
            for i in 0 .. $s.len() {
                __buf[i] = $s[i] as u16;
            }
            __buf[$s.len()] = 0;

            unsafe {
                ::efi::ffi::Print(__buf.as_ptr(), 
                    $($x),*
                );
            }
        }
    };
}
