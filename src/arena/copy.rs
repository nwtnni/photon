use std::alloc;
use std::cell;

use crate::stats;

#[derive(Debug)]
pub struct CopyArena {
    buf: *mut u8,
    cap: usize,
    len: cell::Cell<usize>,
}

impl CopyArena {
    pub fn new(capacity: usize) -> Self {
        unsafe {
            let cap = capacity;
            let len = cell::Cell::new(0);
            let buf = alloc::alloc(alloc::Layout::from_size_align_unchecked(cap, 8));
            CopyArena { cap, len, buf }
        }
    }

    pub fn alloc<T: Copy + std::fmt::Debug>(&self, item: T) -> &T {
        let size = std::mem::size_of::<T>();
        let len = self.len.get();
        if len + size >= self.cap { panic!("[INTERNAL ERROR]: Arena ran out of memory"); }
        self.len.set(len + size);
        stats::ARENA_MEMORY.inc(size);
        unsafe {
            let ptr = self.buf.add(len) as *mut T;
            ptr.write(item);
            &*ptr
        }
    }
}

impl Drop for CopyArena {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(
                self.buf,
                alloc::Layout::from_size_align_unchecked(self.cap, 8)
            );
        }
    }
}
