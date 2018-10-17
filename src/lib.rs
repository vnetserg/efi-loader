#![no_std]

pub mod efi;
use efi::{ EfiStatus, EfiHandle, SystemTable };

efi_main! {
    fn main(handle: EfiHandle, table: SystemTable) -> EfiStatus {
        print!(b"Hello, world!\n");
        print!(b"2 * 2 is %d + %d == %d\n", 1, 3, 4);
        panic!();
    }
}

