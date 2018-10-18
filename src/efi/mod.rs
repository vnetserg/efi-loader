#![macro_use]

pub mod ffi;
pub mod types;
pub mod ctypes;
pub mod macros;
pub mod system_table;
pub mod memory_map;

pub use self::types::*;
pub use self::system_table::SystemTable;
pub use self::memory_map::MemoryMap;


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!(b"PANIC!\n");
    loop {}
}

