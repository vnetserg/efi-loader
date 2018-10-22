use super::{EfiAllocateType, EfiMemoryType, SystemTable, PGSIZE};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use linked_list_allocator::Heap;
use spin::Mutex;

#[alloc_error_handler]
fn error_handler(_: core::alloc::Layout) -> ! {
    panic!("allocator error");
}

pub struct EfiHeap {
    heap: Mutex<Heap>,
}

impl EfiHeap {
    pub const fn empty() -> EfiHeap {
        EfiHeap {
            heap: Mutex::new(Heap::empty()),
        }
    }

    pub unsafe fn init(&self, table: &SystemTable, n_pages: usize) {
        let addr = table
            .boot_services
            .allocate_pages(
                EfiAllocateType::AnyPages,
                EfiMemoryType::LoaderData,
                n_pages,
            )
            .unwrap_or_else(|_| {
                panic!("failed to allocate heap pages.");
            });
        self.heap.lock().init(addr, n_pages * PGSIZE);
    }
}

unsafe impl GlobalAlloc for EfiHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let res = match self.heap.lock().allocate_first_fit(layout) {
            Ok(allocation) => allocation.as_ptr(),
            Err(_) => 0 as *mut u8,
        };
        res
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.heap
            .lock()
            .deallocate(NonNull::new_unchecked(ptr), layout);
    }
}
