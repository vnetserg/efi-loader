#![macro_use]

macro_rules! print {
    ($($arg:tt)*) => ($crate::efi::print(format_args!($($arg)*)));
}

macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! attempt {
    ( $exp:expr, $msg:literal ) => {
        #[allow(unused_imports)]
        {
            use core::ops::Try;
            match $exp.into_result() {
                Ok(val) => Ok(val),
                Err(val) => {
                    println!($msg);
                    Err(val)
                }
            }?
        }
    };
}

macro_rules! halt {
    () => (loop {
        unsafe { asm!("hlt" :::: "volatile") }
    });
}
