use efi::{EfiFile, EfiStatus, MemoryPtr, SystemTable};
use memory_map::MemoryMap;

pub struct KernelHandler {
    addr: MemoryPtr,
    entry: MemoryPtr,
}

impl KernelHandler {
    pub unsafe fn load_image(
        path: &[u8],
        addr: MemoryPtr,
        max_size: usize,
    ) -> Result<KernelHandler, EfiStatus> {
        let mut file = attempt!(
            Self::open_file(path),
            "failed to find kernel image on disk."
        );
        attempt!(
            Self::load_elf(&mut file, addr, max_size),
            "failed to load elf image."
        );
        let entry = attempt!(
            Self::find_entry(&mut file),
            "failed to locate elf entry point."
        );
        Ok(KernelHandler {
            addr,
            entry: addr + entry,
        })
    }

    fn open_file(_path: &[u8]) -> Result<EfiFile, EfiStatus> {
        Err(EfiStatus::LoadError)
    }

    unsafe fn load_elf(_file: &mut EfiFile, _addr: MemoryPtr, _max_size: usize) -> EfiStatus {
        EfiStatus::LoadError
    }

    fn find_entry(_file: &mut EfiFile) -> Result<MemoryPtr, EfiStatus> {
        Err(EfiStatus::LoadError)
    }

    pub fn jump(self, _stack: MemoryPtr, _table: &SystemTable, _memmap: MemoryMap) -> ! {
        println!("Hello from the Rust kernel!");
        halt!()
    }
}
