//use crate::frame::page_alloc;
//use crate::frame::page_free;
//use core::alloc::{AllocError, Allocator, Layout};
use core::alloc::{GlobalAlloc, Layout};

//struct SlabElement {
//next: Option<NonNull<SlabElement>>,
//}

//struct Slab {
//pool: [Option<NonNull<SlabElement>>; 10],
//}

//fn kmalloc(size: usize) -> *mut u8 {
//0 as *mut u8
//}

//fn kfree(ptr: *mut u8) {}

pub struct SlabAllocator;

unsafe impl GlobalAlloc for SlabAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!("alloc {:?}", layout);
        0 as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        println!("free {:?} {:?}", ptr, layout);
    }
}
