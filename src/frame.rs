use ::core::ptr;

const PAGE_SIZE: i32 = 0x1000;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameEntry {
    allocable: bool,
    size: i32,
    next: Option<ptr::NonNull<FrameEntry>>,
    prev: Option<ptr::NonNull<FrameEntry>>,
    id: i32,
    pub addr: i32,
    pub slab_size: i32,
    pub free_slab: i32,
    pub next_slab: Option<ptr::NonNull<FrameEntry>>,
    pub free_slot: i32,
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
            addr: 0,
            next_slab: None,
            free_slot: 0,
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
    pub const fn new() -> Self {
        Self {
            array: [FrameEntry::new(); 0x10000],
            freelists: [None; 20],
            start_addr: 0,
            end_addr: 0,
            max_idx: 0,
        }
    }

    pub fn init(&mut self, start_addr: i32, end_addr: i32) {
        self.start_addr = start_addr;
        self.end_addr = end_addr;
        self.max_idx = (self.end_addr - self.start_addr) / PAGE_SIZE;
        let total_size_val = log2(self.max_idx);
        for idx in 0..self.max_idx {
            self.array[idx as usize].id = idx;
            self.array[idx as usize].addr = idx * PAGE_SIZE + start_addr;
        }
        self.array[0].allocable = true;
        self.array[0].size = total_size_val;
        self.freelists[total_size_val as usize] = Some(ptr::NonNull::from(&self.array[0]));
    }

    pub fn get_page(&self, addr: i32) -> Option<ptr::NonNull<FrameEntry>> {
        Some(ptr::NonNull::from(
            &self.array[((addr - self.start_addr) / PAGE_SIZE) as usize],
        ))
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

    pub fn page_alloc(&mut self, val: i32) -> Option<ptr::NonNull<FrameEntry>> {
        unsafe {
            if self.freelists[val as usize] == None {
                let mut big_frame = self.page_alloc(val + 1);
                match big_frame.as_mut() {
                    Some(v) => {
                        let next_frame = v.as_mut().id + 1 << val;
                        self.array[next_frame as usize].allocable = true;
                        self.array[next_frame as usize].size = val;
                        self.push(
                            val,
                            &mut Some(ptr::NonNull::from(&self.array[next_frame as usize])),
                        );
                        v.as_mut().size = val;
                    }
                    None => {}
                };
                return big_frame;
            } else {
                let mut res = self.freelists[val as usize];
                match res.as_mut() {
                    Some(v) => v.as_mut().allocable = false,
                    None => {}
                }
                self.freelists[val as usize] = res;
                return res;
            }
        }
    }

    pub fn page_free(&mut self, frame: &mut Option<ptr::NonNull<FrameEntry>>) {
        unsafe {
            match frame.as_mut() {
                Some(v) => {
                    let neighbor_id = v.as_mut().id ^ (1 << v.as_mut().size);
                    if neighbor_id < self.max_idx && self.array[neighbor_id as usize].allocable {
                        let head_id = v.as_mut().id & ((1 << v.as_mut().size) - 1);
                        self.array[head_id as usize].allocable = true;
                        self.array[head_id as usize].size = v.as_mut().size + 1;
                        self.remove(&mut Some(ptr::NonNull::from(
                            &self.array[neighbor_id as usize],
                        )));
                        self.page_free(&mut Some(ptr::NonNull::from(
                            &self.array[head_id as usize],
                        )));
                    } else {
                        v.as_mut().allocable = true;
                        self.push(v.as_mut().size, &mut Some(ptr::NonNull::from(v.as_ref())))
                    }
                }
                None => {}
            };
        }
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
