#![macro_use]

macro_rules! wstr {
    ( $s:literal ) => {{
        let mut buf: [u16; $s.len() + 1] = unsafe { core::mem::uninitialized() };
        for i in 0..$s.len() {
            buf[i] = $s[i] as u16;
        }
        buf[$s.len()] = 0;
        buf
    }};
}

macro_rules! print {
    ( $s:literal $(, $x:expr )* ) => {
        #[allow(unused_unsafe)]
        {
            let wstr = wstr!($s);
            unsafe {
                ::efi::ffi::Print(
                    wstr.as_ptr(),
                    $($x),*
                );
            }
        }
    };
}

macro_rules! attempt {
    ( $exp:expr, $msg:literal ) => {
        #[allow(unused_imports)]
        {
            use core::ops::Try;
            match $exp.into_result() {
                Ok(val) => Ok(val),
                Err(val) => {
                    print!($msg);
                    Err(val)
                }
            }?
        }
    };
}
