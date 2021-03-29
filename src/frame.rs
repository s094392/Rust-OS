#[derive(Copy, Clone)]
#[repr(C)]
struct FrameEntry {
    val: i32,
    next: Option<*mut FrameEntry>,
}

impl FrameEntry {
    const fn new() -> Self {
        Self { val: 0, next: None }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct FrameArray {
    array: [FrameEntry; 0x10000],
}

impl FrameArray {
    const fn new() -> Self {
        Self {
            array: [FrameEntry::new(); 0x10000],
        }
    }
}

static PAGE_SIZE: i32 = 4096;
static FREE_PAGE: i32 = -1;
//static ALLOCATED_PAGE:i32 = -2;

fn log2(n: i32) -> i32 {
    let mut targetlevel = 0;
    loop {
        if n >> targetlevel == 0 {
            break;
        }
        targetlevel += 1;
    }
    targetlevel
}

//static FRAME_ARRAY: [UnsafeCell<*mut FrameEntry>; 0x10000] =
//[UnsafeCell::new(0 as *mut FrameEntry); 0x10000];
static mut FRAME_ARRAY: FrameArray = FrameArray::new();

pub fn buddy_init(start_addr: i32, end_addr: i32) {
    let max_idx = (end_addr - start_addr) / PAGE_SIZE;
    let total_size_val = log2(max_idx);

    unsafe {
        for idx in 0..max_idx {
            FRAME_ARRAY.array[idx as usize].val = FREE_PAGE;
        }
        FRAME_ARRAY.array[0].val = total_size_val;
    }
}
