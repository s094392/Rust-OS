use ::core::ptr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameEntry {
    pub val: i32,
    next: Option<ptr::NonNull<FrameEntry>>,
    id: i32,
}

impl FrameEntry {
    const fn new() -> Self {
        Self {
            val: 0,
            next: None,
            id: 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buddy {
    pub array: [FrameEntry; 0x10000],
    freelists: [Option<ptr::NonNull<FrameEntry>>; 20],
}

impl Buddy {
    const fn new() -> Self {
        Self {
            array: [FrameEntry::new(); 0x10000],
            freelists: [None; 20],
        }
    }

    fn push(&mut self, val: i32, frame: &mut Option<ptr::NonNull<FrameEntry>>) {
        unsafe {
            match frame.as_mut() {
                Some(v) => v.as_mut().next = self.freelists[val as usize],
                None => {}
            }
        }
        self.freelists[val as usize] = *frame;
    }
}

const PAGE_SIZE: i32 = 0x1000;
const FREE_PAGE: i32 = -1;
const ALLOCATED_PAGE: i32 = -2;

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

pub static mut BUDDY: Buddy = Buddy::new();

pub fn buddy_init(start_addr: i32, end_addr: i32) {
    let max_idx = (end_addr - start_addr) / PAGE_SIZE;
    let total_size_val = log2(max_idx);

    unsafe {
        for idx in 0..max_idx {
            BUDDY.array[idx as usize].val = FREE_PAGE;
            BUDDY.array[idx as usize].id = idx;
        }
        BUDDY.array[0].val = total_size_val;
        BUDDY.freelists[total_size_val as usize] = Some(ptr::NonNull::from(&BUDDY.array[0]));
    }
}

pub fn page_alloc(val: i32) -> Option<ptr::NonNull<FrameEntry>> {
    unsafe {
        if BUDDY.freelists[val as usize] == None {
            let mut big_frame = page_alloc(val + 1);
            match big_frame.as_mut() {
                Some(v) => {
                    let next_frame = v.as_mut().id + 1 << val;
                    BUDDY.array[next_frame as usize].val = val;
                    BUDDY.push(
                        val,
                        &mut Some(ptr::NonNull::from(&BUDDY.array[next_frame as usize])),
                    );
                }
                None => {}
            };
            return big_frame;
        } else {
            let mut res = BUDDY.freelists[val as usize];
            match res.as_mut() {
                Some(v) => v.as_mut().val = ALLOCATED_PAGE,
                None => {}
            }
            BUDDY.freelists[val as usize] = res;
            return res;
        }
    }
}
