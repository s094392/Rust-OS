use crate::frame::Buddy;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

struct SlabElement {
    addr: i32,
    next: Option<NonNull<SlabElement>>,
}

struct Slab {
    pool: [Option<NonNull<SlabElement>>; 12],
}

impl Slab {
    pub const fn new() -> Slab {
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

static mut SLAB: Slab = Slab::new();
static mut BUDDY: Buddy = Buddy::new();

pub struct PiAllocator {}

impl PiAllocator {
    pub const fn new() -> PiAllocator {
        PiAllocator {}
    }
    //0x1000_0000, 0x2000_0000
    pub unsafe fn init(&self) {
        BUDDY.init(0x1000_0000, 0x2000_0000);
    }
}

unsafe impl GlobalAlloc for PiAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let order = log2(layout.size() as i32);
        if order >= 12 {
            let page = BUDDY.page_alloc(order);
            page.unwrap().as_ref().addr as *mut u8
        } else {
            match SLAB.pool[order as usize] {
                Some(v) => {
                    SLAB.pool[order as usize] = v.as_ref().next;
                    v.as_ref().addr as *mut u8
                }
                None => {
                    let page = BUDDY.page_alloc(0);
                    page.unwrap().as_mut().slab_size = 4096 / layout.size() as i32;
                    page.unwrap().as_mut().free_slab = 4096 / layout.size() as i32;
                    self.alloc(layout)
                }
            }
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let order = log2(layout.size() as i32);
        if order >= 12 {
            let mut page = BUDDY.get_page(ptr as i32);
            BUDDY.page_free(&mut page);
        } else {
            println!("fo");
        }
    }
}
