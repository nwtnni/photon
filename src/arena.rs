use std::alloc;
use std::cell;

use crate::stats;

#[derive(Debug)]
pub struct Arena {
    buf: *mut u8,
    cap: usize,
    len: cell::Cell<usize>,
}

impl Arena {
    pub fn new(capacity: usize) -> Self {
        unsafe {
            let cap = capacity;
            let len = cell::Cell::new(0);
            let buf = alloc::alloc(alloc::Layout::from_size_align_unchecked(cap, 8));
            Arena { cap, len, buf }
        }
    }

    pub fn alloc<T: Copy>(&self, item: T) -> &T {
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

    pub unsafe fn alloc_slice_uninitialized<T: Copy>(&self, count: usize) -> &mut [T] {
        let size = std::mem::size_of::<T>();
        let len = self.len.get();
        if len + size * count >= self.cap { panic!("[INTERNAL ERROR]: Arena ran out of memory"); }
        self.len.set(len + size * count);
        stats::ARENA_MEMORY.inc(size * count);
        let ptr = self.buf.add(len) as *mut T;
        std::slice::from_raw_parts_mut(ptr, count)
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(
                self.buf,
                alloc::Layout::from_size_align_unchecked(self.cap, 8)
            );
        }
    }
}
