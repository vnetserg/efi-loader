#![no_std]

pub mod efi;
pub use efi::{ EfiStatus, EfiHandle, SystemTable };

efi_main! {
    fn main(handle: EfiHandle, table: SystemTable) -> EfiStatus {
        print!(b"Hello, world!\n");
        print!(b"2 * 2 is %d\n", 4);
        panic!();
    }
}

