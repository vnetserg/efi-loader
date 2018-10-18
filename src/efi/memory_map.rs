use super::EfiStatus;
use super::SystemTable;

pub struct MemoryMap {}

impl MemoryMap {
    pub fn current(table: &SystemTable) -> Result<MemoryMap, EfiStatus> {
        Err(EfiStatus::EfiLoadError)
    }
}
