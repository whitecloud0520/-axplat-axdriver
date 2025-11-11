use core::cmp;

use heapless::Vec;
use spin::Mutex;

use crate::api::{DmaRegion, MemIf};

const BASE_PADDR: usize = 0x8000_0000;

pub struct MockMemory {
    next_paddr: Mutex<usize>,
    allocations: Mutex<Vec<DmaRegion, 16>>,
}

impl MockMemory {
    pub const fn new() -> Self {
        Self {
            next_paddr: Mutex::new(BASE_PADDR),
            allocations: Mutex::new(Vec::new()),
        }
    }

    pub fn allocations(&self) -> Vec<DmaRegion, 16> {
        self.allocations.lock().clone()
    }

    #[cfg(test)]
    pub fn reset(&self) {
        *self.next_paddr.lock() = BASE_PADDR;
        self.allocations.lock().clear();
    }
}

impl MemIf for MockMemory {
    fn dma_alloc(&self, bytes: usize, align: usize) -> Option<DmaRegion> {
        let align = cmp::max(1, align);
        let mut next = self.next_paddr.lock();
        let aligned = (*next + align - 1) & !(align - 1);
        let region = DmaRegion::new(aligned, bytes, align);
        *next = aligned + bytes;

        let mut slots = self.allocations.lock();
        let _ = slots.push(region);

        Some(region)
    }

    fn dma_flush(&self, _region: &DmaRegion) {
        // mock flush is a no-op
    }
}

pub static PLATFORM_MEMORY: MockMemory = MockMemory::new();

pub fn platform_memory() -> &'static dyn MemIf {
    &PLATFORM_MEMORY
}

#[cfg(test)]
pub fn reset() {
    PLATFORM_MEMORY.reset();
}
