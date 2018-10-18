use super::EfiStatus;
use super::SystemTable;

pub struct MemoryMap {}

impl MemoryMap {
    pub fn current(table: &SystemTable) -> Result<MemoryMap, EfiStatus> {
        Ok(MemoryMap{})
    }

    pub fn find(&self, k: i32, min_size: usize)
            -> Result<usize, EfiStatus> {
        Err(EfiStatus::EfiLoadError)
    }
}
