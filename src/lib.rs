#![no_std]
#![feature(try_trait)]
#![feature(const_slice_len)]
#![feature(macro_literal_matcher)]

mod efi;
use efi::{ EfiStatus, EfiHandle, MemoryMap, SystemTable };

const KERNEL_IMAGE_PATH: &[u8] = b"\\EFI\\BOOT\\KERNEL.IMG";

efi_main! {
    fn main(handle: EfiHandle, table: SystemTable) -> EfiStatus {
        print!(b"OS loader started.\n");

        print!(b"Getting memory map... ");
        let memmap = MemoryMap::current(&table)?;
        print!(b"done.\n");

        print!(b"Looking for free space... ");
        let kaddr = memmap.find(0, 4096)?; // kernel address
        let saddr = memmap.find(1, 4096)?; // kernel stack address
        print!(b" found.\n");

        return EfiStatus::EfiSuccess;
    }
}

