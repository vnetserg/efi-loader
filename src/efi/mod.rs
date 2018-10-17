#![macro_use]

pub mod ffi;
pub mod types;
pub mod ctypes;
pub mod macros;
pub mod system_table;

pub use self::types::*;
pub use self::system_table::SystemTable;


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print(b"PANIC!\n");
    loop {}
}

pub fn print(s: &[u8]) {
    let mut buf: [u16; 2048] = [0; 2048];
    for i in 0 .. s.len() {
        buf[i] = s[i] as u16;
    }

    unsafe {
        ffi::Print(buf.as_ptr());
    }
}
