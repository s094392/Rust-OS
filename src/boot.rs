mod boot;
mod memory;

pub unsafe fn clear_bss() {
    let bss_range = memory::bss_range_inclusive();
    memory::zero_volatile(bss_range);
}
