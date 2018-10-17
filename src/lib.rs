#![no_std]

pub mod efi;
pub use efi::{ EfiStatus, EfiHandle, SystemTable, print };


efi_main! {
    fn main(handle: EfiHandle, table: SystemTable) -> EfiStatus {
        print(b"Hello, world!\n");
        return EfiStatus::EfiSuccess;
    }
}

