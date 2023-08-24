//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator{
    start:usize,
    next:usize,
    allocation:usize,
    end:usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start:0,
            next:0,
            allocation:0,
            end:0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        // todo!();
        self.start = start;
        self.next = self.start;
        self.end = self.start + size;
        self.allocation = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        // todo!();
        Err(crate::AllocError::NoMemory)
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        // todo!();
        let size = layout.size();
        let align = layout.align();
        let align_mask = !(align - 1);

        let start = (self.next + align - 1) & (align_mask);

        if start + size > self.end{
            Err(crate::AllocError::NoMemory)
        } else{
            self.allocation += 1;
            self.next = start + size;
            Ok(NonZeroUsize::new(start).unwrap())
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        // todo!();
        self.allocation -= 1;
        if self.allocation == 0{
            self.next = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        // todo!();
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        // todo!();
        self.next - self.start
    }

    fn available_bytes(&self) -> usize {
        // todo!();
        self.end - self.next
    }
}
