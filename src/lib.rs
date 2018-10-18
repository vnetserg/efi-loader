#![no_std]
#![feature(try_trait)]
#![feature(const_slice_len)]
#![feature(macro_literal_matcher)]

mod efi;
mod kernel_handler;

use efi::{ EfiStatus, EfiHandle, MemoryMap, MemoryQuery, SystemTable };
use kernel_handler::KernelHandler;

const KERNEL_IMAGE_PATH: &[u8] = b"\\EFI\\BOOT\\KERNEL.IMG";

efi_main! {
    fn main(handle: EfiHandle, table: SystemTable) -> EfiStatus {
        print!(b"UEFI OS loader started.\n");

        print!(b"Getting memory map... ");
        let mut memmap = MemoryMap::current(&table)?;
        print!(b"done.\n");

        print!(b"Looking for free space... ");
        let kseg = memmap.find_segment(MemoryQuery::KthBiggest(0))?;
        let sseg = memmap.find_segment(MemoryQuery::KthBiggest(1))?;
        print!(b"found.\n");

        print!(b"Loading kernel image... ");
        let khandler = KernelHandler::load_image(KERNEL_IMAGE_PATH,
                                                 kseg.start,
                                                 kseg.size)?;
        print!(b"done.\n");

        print!(b"Exiting boot services... ");
        exit_boot_services(handle, &table, &mut memmap);
        print!(b"done.\n");

        print!(b"Jumping into the kernel. Goodbye, UEFI!\n");
        khandler.jump(sseg.start, &memmap); // never returns
    }
}

fn exit_boot_services(_handle: EfiHandle, _table: &SystemTable,
                      _memmap: &MemoryMap) -> EfiStatus {
    EfiStatus::Success
}
