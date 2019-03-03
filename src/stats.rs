use std::sync::atomic::{AtomicUsize, Ordering};

macro_rules! counter {
    ($name:ident) => {
        pub static $name: Counter = Counter {
            name: stringify!($name),
            value: AtomicUsize::new(0),
        };
    }
}

macro_rules! memory {
    ($name:ident) => {
        pub static $name: Memory = Memory {
            name: stringify!($name),
            value: AtomicUsize::new(0),
        };
    }
}

counter!(PIXELS_RENDERED);
counter!(TOTAL_NODES);
counter!(LEAF_NODES);
counter!(INTERSECTION_TESTS);
counter!(BOUNDING_BOX_INTERSECTION_TESTS);
counter!(SPHERE_INTERSECTION_TESTS);
counter!(LIST_INTERSECTION_TESTS);
memory!(ARENA_MEMORY);

#[derive(Debug)]
pub struct Counter {
    name: &'static str,
    value: AtomicUsize,
}

impl Counter {
    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    pub fn read(&self) -> usize {
        self.value.load(Ordering::Acquire)
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = self.value.load(Ordering::Acquire);
        write!(fmt, "{}: {}", self.name, value)
    }
}

#[derive(Debug)]
pub struct Memory {
    name: &'static str,
    value: AtomicUsize,
}

const GB: usize = 1_073_741_824;
const MB: usize = 1_048_576;
const KB: usize = 1_024;

impl Memory {
    pub fn inc(&self, value: usize) {
        self.value.fetch_add(value, Ordering::Relaxed);
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.value.load(Ordering::Acquire);
        let gb = bytes / GB; bytes %= GB;
        let mb = bytes / MB; bytes %= MB;
        let kb = bytes / KB; bytes %= KB;
        let mut format = String::new();
        if gb > 0 {
            format += &format!("{}GB ", gb);
        } else if mb > 0 {
            format += &format!("{}MB ", mb);
        } else if kb > 0 {
            format += &format!("{}KB ", kb);
        } else if bytes > 0{
            format += &format!("{}B ", bytes);
        }
        write!(fmt, "{}: {}", self.name, format)
    }
}
