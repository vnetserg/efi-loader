use super::ffi::Print;
use alloc::vec::Vec;
use core::fmt;
use core::fmt::Write;

struct Console {}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut wstr: Vec<u16> = Vec::with_capacity(s.len() + 1);
        for &c in s.as_bytes() {
            wstr.push(c as u16);
        }
        wstr.push(0);
        unsafe {
            Print(wstr.as_ptr());
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Console {}.write_fmt(args).unwrap();
}
