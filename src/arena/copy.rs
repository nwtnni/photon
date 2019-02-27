use std::alloc;
use std::cell;

#[derive(Debug)]
pub struct CopyArena {
    cap: cell::Cell<usize>,
    len: cell::Cell<usize>,
    buf: cell::Cell<*mut u8>,
}

impl CopyArena {
    pub fn new(capacity: usize) -> Self {
        unsafe {
            let cap = cell::Cell::new(capacity);
            let len = cell::Cell::new(0);
            let buf = cell::Cell::new(alloc::alloc(
                alloc::Layout::from_size_align_unchecked(capacity, 8)
            ));
            CopyArena { cap, len, buf }
        }
    }

    pub fn alloc<T: Copy>(&self, item: T) -> &T {
        let size = std::mem::size_of::<T>();
        let len = self.len.get();
        let cap = self.cap.get();
        if len + size >= cap { panic!("[INTERNAL ERROR]: Arena ran out of memory"); }
        unsafe {
            let ptr = self.buf.get() as *mut T;
            ptr.write_unaligned(item);
            self.len.set(len + size);
            self.buf.set(ptr.add(size) as *mut u8);
            &*ptr
        }
    }
}

impl Drop for CopyArena {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(
                self.buf.get(),
                alloc::Layout::from_size_align_unchecked(self.cap.get(), 8)
            );
        }
    }
}
