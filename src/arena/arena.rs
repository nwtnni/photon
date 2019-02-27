use std::alloc;
use std::cell;

pub struct Arena<T> {
    buf: *mut T,
    cap: usize,
    len: cell::Cell<usize>,
}

impl<T> Arena<T> {
    pub fn new(capacity: usize) -> Self {
        unsafe {
            let cap = capacity;
            let len = cell::Cell::new(0);
            let buf = alloc::alloc(alloc::Layout::from_size_align_unchecked(cap, 8)) as *mut T;
            Arena { cap, len, buf }
        }
    }

    pub fn alloc(&self, item: T) -> &T {
        let len = self.len.get();
        if len + 1 >= self.cap { panic!("[INTERNAL ERROR]: Arena ran out of memory"); }
        self.len.set(len + 1);
        unsafe {
            let ptr = self.buf.add(len);
            ptr.write(item);
            &*ptr
        }
    }
}

impl<T> Drop for Arena<T> {
    fn drop(&mut self) {
        unsafe {
            // Call destructors
            for i in (0..self.len.get()).rev() {
                std::ptr::read(self.buf.add(i) as *const T);
            }
            alloc::dealloc(
                self.buf as *mut u8,
                alloc::Layout::from_size_align_unchecked(self.cap, 8),
            );
        }
    }
}
