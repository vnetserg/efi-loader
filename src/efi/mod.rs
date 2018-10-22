#![macro_use]

pub mod heap;
pub mod ctypes;
pub mod ffi;
pub mod macros;
pub mod system_table;
pub mod types;

pub use self::heap::EfiHeap;
pub use self::system_table::{BootServices, EfiMemoryDescriptorArray, SystemTable};
pub use self::types::*;
use alloc::prelude::ToString;
use core::panic::PanicInfo;

pub const PGSIZE: usize = 4096; // memory page size in bytes

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(msg) = info.message() {
        let text = msg.to_string();
        print!(b"PANIC: %s\n", text.as_ptr());
    } else {
        print!(b"PANIC!\n");
    }
    loop {}
}
