use ::core::ptr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameEntry {
    allocable: bool,
    size: i32,
    next: Option<ptr::NonNull<FrameEntry>>,
    prev: Option<ptr::NonNull<FrameEntry>>,
    id: i32,
    pub slab_size: i32,
    pub free_slab: i32,
}

impl FrameEntry {
    const fn new() -> Self {
        Self {
            allocable: false,
            size: 0,
            next: None,
            prev: None,
            id: 0,
            slab_size: 0,
            free_slab: 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buddy {
    pub array: [FrameEntry; 0x10000],
    freelists: [Option<ptr::NonNull<FrameEntry>>; 20],
    start_addr: i32,
    end_addr: i32,
    max_idx: i32,
}

impl Buddy {
    const fn new() -> Self {
        Self {
            array: [FrameEntry::new(); 0x10000],
            freelists: [None; 20],
            start_addr: 0,
            end_addr: 0,
            max_idx: 0,
        }
    }

    fn push(&mut self, val: i32, frame: &mut Option<ptr::NonNull<FrameEntry>>) {
        unsafe {
            match self.freelists[val as usize] {
                Some(mut v) => {
                    v.as_mut().prev = *frame;
                    frame.unwrap().as_mut().next = self.freelists[val as usize];
                }
                None => {}
            }
        }
        self.freelists[val as usize] = *frame;
    }

    fn remove(&mut self, frame: &mut Option<ptr::NonNull<FrameEntry>>) {
        unsafe {
            let current = frame.unwrap();
            let prev = current.as_ref().prev;
            let next = current.as_ref().next;
            match prev {
                Some(_v) => {
                    prev.unwrap().as_mut().next = next;
                }
                None => {
                    self.freelists[current.as_ref().size as usize] = next;
                }
            }
            match next {
                Some(_v) => {
                    next.unwrap().as_mut().prev = prev;
                }
                None => {}
            }
        }
    }
}

const PAGE_SIZE: i32 = 0x1000;

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

static mut BUDDY: Buddy = Buddy::new();

pub fn buddy_init(start_addr: i32, end_addr: i32) {
    unsafe {
        BUDDY.start_addr = start_addr;
        BUDDY.end_addr = end_addr;
        BUDDY.max_idx = (BUDDY.end_addr - BUDDY.start_addr) / PAGE_SIZE;
        let total_size_val = log2(BUDDY.max_idx);
        for idx in 0..BUDDY.max_idx {
            BUDDY.array[idx as usize].id = idx;
        }
        BUDDY.array[0].allocable = true;
        BUDDY.array[0].size = total_size_val;
        BUDDY.freelists[total_size_val as usize] = Some(ptr::NonNull::from(&BUDDY.array[0]));
    }
}

pub fn page_alloc(val: i32) -> Option<ptr::NonNull<FrameEntry>> {
    println!("alloc {}", val);
    unsafe {
        if BUDDY.freelists[val as usize] == None {
            println!("alloc bigger {}", val + 1);
            let mut big_frame = page_alloc(val + 1);
            match big_frame.as_mut() {
                Some(v) => {
                    let next_frame = v.as_mut().id + 1 << val;
                    BUDDY.array[next_frame as usize].allocable = true;
                    BUDDY.array[next_frame as usize].size = val;
                    BUDDY.push(
                        val,
                        &mut Some(ptr::NonNull::from(&BUDDY.array[next_frame as usize])),
                    );
                    v.as_mut().size = val;
                }
                None => {}
            };
            return big_frame;
        } else {
            let mut res = BUDDY.freelists[val as usize];
            match res.as_mut() {
                Some(v) => v.as_mut().allocable = false,
                None => {}
            }
            BUDDY.freelists[val as usize] = res;
            return res;
        }
    }
}

pub fn page_free(frame: &mut Option<ptr::NonNull<FrameEntry>>) {
    unsafe {
        match frame.as_mut() {
            Some(v) => {
                let neighbor_id = v.as_mut().id ^ (1 << v.as_mut().size);
                if neighbor_id < BUDDY.max_idx && BUDDY.array[neighbor_id as usize].allocable {
                    let head_id = v.as_mut().id & ((1 << v.as_mut().size) - 1);
                    println!("merge {}, {} -> {}", v.as_mut().id, neighbor_id, head_id);
                    BUDDY.array[head_id as usize].allocable = true;
                    BUDDY.array[head_id as usize].size = v.as_mut().size + 1;
                    BUDDY.remove(&mut Some(ptr::NonNull::from(
                        &BUDDY.array[neighbor_id as usize],
                    )));
                    page_free(&mut Some(ptr::NonNull::from(
                        &BUDDY.array[head_id as usize],
                    )));
                } else {
                    println!("push {}", v.as_mut().size);
                    v.as_mut().allocable = true;
                    BUDDY.push(v.as_mut().size, &mut Some(ptr::NonNull::from(v.as_ref())))
                }
            }
            None => {}
        };
    }
}
