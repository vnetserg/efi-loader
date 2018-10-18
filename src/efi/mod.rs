#![macro_use]

pub mod ffi;
pub mod types;
pub mod ctypes;
pub mod macros;
pub mod system_table;

pub use self::types::*;
pub use self::system_table::{ SystemTable, BootServices,
                              EfiMemoryDescriptorArray };

pub const PGSIZE: usize = 4096; // memory page size in bytes


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!(b"PANIC!\n");
    loop {}
}

