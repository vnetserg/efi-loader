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
    unsafe {
        ffi::Print(b"P\0A\0N\0I\0C\0!\0\n\0\0\0".as_ptr() as *const u16);
    }
    loop {}
}

