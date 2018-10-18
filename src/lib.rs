#![no_std]
#![feature(try_trait)]
#![feature(const_slice_len)]
#![feature(macro_literal_matcher)]

mod efi;
use efi::{ EfiStatus, EfiHandle, MemoryMap, SystemTable };

const KERNEL_IMAGE_PATH: &[u8] = b"\\EFI\\BOOT\\KERNEL.IMG";

efi_main! {
    fn main(handle: EfiHandle, table: SystemTable) -> EfiStatus {
        print!(b"OS loader started\n");

        // Get memory map
        let memmap = MemoryMap::current(&table)?;
        print!(b"Got memory map\n");

        return EfiStatus::EfiSuccess;
    }
}

