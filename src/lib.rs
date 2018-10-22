#![no_std]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(try_trait)]
#![feature(const_slice_len)]
#![feature(panic_info_message)]
#![feature(stmt_expr_attributes)]
#![feature(macro_literal_matcher)]

extern crate alloc;
extern crate spin;
extern crate linked_list_allocator;

mod efi;
mod kernel_handler;
mod memory_map;

use efi::{EfiHandle, EfiHeap, EfiStatus, SystemTable};
use kernel_handler::KernelHandler;
use memory_map::{MemoryMap, MemoryQuery};

const HEAP_PAGES: usize = 10;
const KERNEL_IMAGE_PATH: &[u8] = b"\\EFI\\BOOT\\KERNEL.IMG";

#[global_allocator]
static ALLOCATOR: EfiHeap = EfiHeap::empty();

#[no_mangle]
pub extern "C" fn efi_main(
    handle: efi::ctypes::EFI_HANDLE,
    st: *const efi::ctypes::EFI_SYSTEM_TABLE,
) -> ! {
    unsafe {
        efi::ffi::InitializeLib(handle, st);
    }
    let table = unsafe { efi::SystemTable::claim(st) };
    unsafe {
        ALLOCATOR.init(table, HEAP_PAGES);
    }
    let status = main(handle, table);
    print!(
        b"Loader exited. Status: %r.\n",
        status as efi::ctypes::EFI_STATUS
    );
    loop {}
}

fn main(handle: EfiHandle, table: &SystemTable) -> EfiStatus {
    print!(b"UEFI OS loader started.\n");

    print!(b"Getting memory map... ");
    let memmap = MemoryMap::current(table)?;
    print!(b"done.\n");

    print!(b"Looking for free space... ");
    let kseg = memmap.find_segment(MemoryQuery::KthBiggest(0))?;
    let sseg = memmap.find_segment(MemoryQuery::KthBiggest(1))?;
    print!(b"found.\n");

    print!(b"Loading kernel image... ");
    let khandler = KernelHandler::load_image(KERNEL_IMAGE_PATH, kseg.start, kseg.size)?;
    print!(b"done.\n");

    print!(b"Exiting boot services... ");
    let memmap = exit_boot_services(handle, table, memmap)?;
    print!(b"done.\n");

    print!(b"Jumping into the kernel. Goodbye, UEFI!\n");
    khandler.jump(sseg.start, table, memmap); // never returns
}

fn exit_boot_services(
    _handle: EfiHandle,
    _table: &SystemTable,
    memmap: MemoryMap,
) -> Result<MemoryMap, EfiStatus> {
    Ok(memmap)
}
