use std::alloc;
use std::cell;

use crate::stats;

const ALIGN: usize = 8;

#[derive(Debug)]
pub struct Arena {
    buf: *mut u8,
    cap: usize,
    len: cell::Cell<usize>,
}

impl Arena {
    pub fn new(capacity: usize) -> Self {
        let cap = capacity;
        let len = cell::Cell::new(0);
        let lay = alloc::Layout::from_size_align(cap, ALIGN).unwrap();
        unsafe {
            let buf = alloc::alloc(lay);
            Arena { cap, len, buf }
        }
    }

    pub fn alloc<T: Copy>(&self, item: T) -> &T {
        let size = std::mem::size_of::<T>();
        let size = (size + ALIGN - 1) & !(ALIGN - 1);
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
