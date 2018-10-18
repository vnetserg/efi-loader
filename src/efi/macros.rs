#![macro_use]

macro_rules! efi_main {
    ($(fn main($handle:ident: $htype:ty, $table:ident: $stype:ty)
            -> $ret:ty $body:block)+) => ($(

        fn main($handle: $htype, $table: $stype) -> $ret
            $body
                
        #[no_mangle]
        pub extern "stdcall" fn efi_main(handle: efi::ctypes::EFI_HANDLE,
                                         st: *const efi::ctypes::EFI_SYSTEM_TABLE)
                -> ! {
            unsafe {
                efi::ffi::InitializeLib(handle, st);
            }
            let table = efi::SystemTable::new(st);
            let status = main(handle, table);
            print!(b"Exit status: %r\n", status as efi::ctypes::EFI_STATUS);
            loop {}
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
