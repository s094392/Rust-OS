use core::cell::UnsafeCell;
use core::ops::RangeInclusive;

extern "Rust" {
    static __bss_start: UnsafeCell<u64>;
    static __bss_end: UnsafeCell<u64>;
}

pub fn bss_range_inclusive() -> RangeInclusive<*mut u64> {
    let range;
    unsafe {
        range = RangeInclusive::new(__bss_start.get(), __bss_end.get());
    }
    assert!(!range.is_empty());

    range
}

pub unsafe fn zero_volatile<T>(range: RangeInclusive<*mut T>)
where
    T: From<u8>,
{
    let mut ptr = *range.start();
    let end_inclusive = *range.end();

    while ptr <= end_inclusive {
        core::ptr::write_volatile(ptr, T::from(0));
        ptr = ptr.offset(1);
    }
}

