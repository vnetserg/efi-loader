#![macro_use]

pub mod console;
pub mod ctypes;
pub mod ffi;
pub mod heap;
pub mod macros;
pub mod system_table;
pub mod types;

pub use self::console::print;
pub use self::heap::EfiHeap;
pub use self::system_table::{BootServices, SystemTable};
pub use self::types::*;

use core::panic::PanicInfo;

pub const PGSIZE: usize = 4096; // memory page size in bytes

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info);
    halt!();
}
