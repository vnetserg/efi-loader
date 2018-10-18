use super::EfiStatus;
use super::SystemTable;

pub struct MemoryMap {}

pub struct MemorySegment {
    pub start: usize,
    pub size: usize
}

pub enum MemoryQuery {
    KthBiggest(usize)
}

impl MemoryMap {
    pub fn current(_table: &SystemTable) -> Result<MemoryMap, EfiStatus> {
        Ok(MemoryMap{})
    }

    pub fn find_segment(&self, _query: MemoryQuery)
            -> Result<MemorySegment, EfiStatus> {
        Ok(MemorySegment{ start: 0, size: 0})
    }
}
