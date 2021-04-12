use crate::frame::{Buddy, FrameEntry};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

struct Slab {
    pool: [Option<NonNull<FrameEntry>>; 12],
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

    pub unsafe fn init(&self) {
        BUDDY.init(0x1000_0000, 0x2000_0000);
    }

    unsafe fn set_value(addr: *mut i32, value: i32) {
        *addr = value;
    }

    unsafe fn get_value(addr: *mut i32) -> i32 {
        *addr
    }

    unsafe fn get_free_pool(
        slab: Option<NonNull<FrameEntry>>,
        layout: Layout,
    ) -> NonNull<FrameEntry> {
        let order = log2(layout.size() as i32);
        match slab {
            Some(v) => {
                if v.as_ref().free_slab > 0 {
                    v
                } else {
                    println!("Slab_pool full");
                    PiAllocator::get_free_pool(v.as_ref().next_slab, layout)
                }
            }
            None => {
                println!("Slabpool {} is none", order);
                let page = BUDDY.page_alloc(0);
                let slab_size = PAGE_SIZE / layout.size() as i32;
                page.unwrap().as_mut().slab_size = slab_size;
                page.unwrap().as_mut().free_slab = slab_size;
                for idx in 0..slab_size - 1 {
                    PiAllocator::set_value(
                        (idx * layout.size() as i32 + page.unwrap().as_ref().addr) as *mut i32,
                        idx + 1,
                    );
                }
                println!("inited");
                page.unwrap().as_mut().next_slab = SLAB.pool[order as usize];
                SLAB.pool[order as usize] = page;
                page.unwrap()
            }
        }
    }

    unsafe fn free_slab_page(order: i32) {
        let mut page = SLAB.pool[order as usize];
        if page.is_some() {
            if page.unwrap().as_ref().free_slab == page.unwrap().as_ref().slab_size {
                SLAB.pool[order as usize] = page.unwrap().as_ref().next_slab;
                BUDDY.page_free(&mut page);
            } else {
                while page.unwrap().as_ref().next_slab.is_some() {
                    if page.unwrap().as_ref().next_slab.unwrap().as_ref().free_slab
                        == page.unwrap().as_ref().next_slab.unwrap().as_ref().slab_size
                    {
                        page.unwrap().as_mut().next_slab =
                            page.unwrap().as_ref().next_slab.unwrap().as_ref().next_slab;
                        BUDDY.page_free(&mut page.unwrap().as_mut().next_slab);
                        return;
                    }
                    page = page.unwrap().as_ref().next_slab;
                }
            }
        }
    }
}

const PAGE_SIZE: i32 = 0x1000;

unsafe impl GlobalAlloc for PiAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let order = log2(layout.size() as i32);
        println!("Alloc: {}", order);
        if order >= 12 {
            let page = BUDDY.page_alloc(order - 12);
            page.unwrap().as_ref().addr as *mut u8
        } else {
            println!("Alloc from slab");
            let mut slab_pool = PiAllocator::get_free_pool(SLAB.pool[order as usize], layout);
            let res = slab_pool.as_ref().free_slot;
            slab_pool.as_mut().free_slot = PiAllocator::get_value(
                (res * layout.size() as i32 + slab_pool.as_ref().addr) as *mut i32,
            );
            slab_pool.as_mut().free_slab -= 1;
            let addr = (slab_pool.as_ref().addr + res * layout.size() as i32) as *mut u8;
            println!(
                "allocated: {:#x}, next: {:#x}",
                addr as i32,
                slab_pool.as_mut().free_slot
            );
            addr
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let order = log2(layout.size() as i32);
        if order >= 12 {
            let mut page = BUDDY.get_page(ptr as i32);
            BUDDY.page_free(&mut page);
        } else {
            let page = BUDDY.get_page(ptr as i32);
            page.unwrap().as_mut().free_slab += 1;
            let slot_id = (ptr as i32 - page.unwrap().as_ref().addr) / (1 << order);
            PiAllocator::set_value(ptr as *mut i32, page.unwrap().as_mut().free_slot);
            page.unwrap().as_mut().free_slot = slot_id;
            if page.unwrap().as_ref().slab_size == page.unwrap().as_ref().free_slab {
                PiAllocator::free_slab_page(order);
            }
        }
    }
}
