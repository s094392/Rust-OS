//use crate::frame::page_alloc;
//use crate::frame::page_free;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

struct SlabElement {
    next: Option<NonNull<SlabElement>>,
}

struct Slab {
    pool: [Option<NonNull<SlabElement>>; 12],
}

impl Slab {
    pub fn new() -> Slab {
        Slab { pool: [None; 12] }
    }
}

fn log2(n: i32) -> i32 {
    let mut targetlevel = 0;
    loop {
        if n >> targetlevel == 0 {
            break;
        }
        targetlevel += 1;
    }
    targetlevel - 1
}

fn kmalloc(size: usize) -> *mut u8 {
    println!("alloc {:?}", size);
    let order = log2(size as i32);
    0 as *mut u8
}

fn kfree(ptr: *mut u8, size: usize) {
    println!("free {:?} {:?}", ptr, size);
}

pub struct PiAllocator {
    slab: Slab,
}

impl PiAllocator {
    pub fn new() -> PiAllocator {
        PiAllocator { slab: Slab::new() }
    }
}

unsafe impl GlobalAlloc for PiAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        kmalloc(layout.size())
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        kfree(ptr, layout.size());
    }
}
