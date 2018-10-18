use efi::{ MemoryMap, EfiStatus };

pub struct KernelHandler {}

impl KernelHandler {
    pub fn load_image(_path: &[u8], _addr: usize, _max_size: usize)
            -> Result<KernelHandler, EfiStatus> {
        Ok(KernelHandler{})
    }

    pub fn jump(self, _stack: usize, _memmap: &MemoryMap) -> ! {
        print!(b"Hello from the Rust kernel!\n");
        loop {}
    }
}
