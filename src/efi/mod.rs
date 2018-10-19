#![macro_use]

pub mod ctypes;
pub mod ffi;
pub mod macros;
pub mod system_table;
pub mod types;

pub use self::system_table::{BootServices, EfiMemoryDescriptorArray, SystemTable};
pub use self::types::*;

pub const PGSIZE: usize = 4096; // memory page size in bytes

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!(b"PANIC!\n");
    loop {}
}
