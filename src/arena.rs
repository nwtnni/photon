use std::alloc;
use std::cell;
use std::slice;

use crate::stats;

const DEFAULT_CAPACITY: usize = 64 * 1024 * 1024;
const DEFAULT_ALIGNMENT: usize = 8;

#[derive(Debug)]
pub struct Arena {
    buf: *mut u8,
    cap: usize,
    len: cell::Cell<usize>,
    align: usize,
}

impl Arena {
    pub fn new(capacity: usize, align: usize) -> Self {
        let cap = capacity;
        let len = cell::Cell::new(0);
        let lay = alloc::Layout::from_size_align(capacity, align).unwrap();
        unsafe {
            let buf = alloc::alloc(lay);
            Arena { cap, len, buf, align }
        }
    }

    pub fn alloc<T: Copy>(&self, item: T) -> &T {
        let size = self.align(std::mem::size_of::<T>());
        let len = self.len.get();
        if len + size >= self.cap { panic!("[INTERNAL ERROR]: arena ran out of memory"); }
        self.len.set(len + size);
        stats::ARENA_MEMORY.inc(size);
        unsafe {
            let ptr = self.buf.add(len) as *mut T;
            ptr.write(item);
            &*ptr
        }
    }

    pub unsafe fn alloc_slice_mut<T: Copy>(&self, count: usize) -> &mut [T] {
        let size = self.align(std::mem::size_of::<T>()) * count; 
        let len = self.len.get();
        if len + size >= self.cap { panic!("[INTERNAL ERROR]: arena ran out of memory"); }
        self.len.set(len + size);
        stats::ARENA_MEMORY.inc(size);
        let ptr = self.buf.add(len) as *mut T;
        let len = count;
        std::slice::from_raw_parts_mut(ptr, len)
    }

    fn align(&self, addr: usize) -> usize {
        (addr + self.align - 1) & !(self.align - 1)
    }

    fn layout(&self) -> alloc::Layout {
        alloc::Layout::from_size_align(self.cap, self.align).unwrap()
    }
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new(DEFAULT_CAPACITY, DEFAULT_ALIGNMENT)
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.buf, self.layout());
        }
    }
}
