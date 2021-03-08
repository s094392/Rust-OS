#![feature(asm)]
#![feature(global_asm)]
#![feature(const_fn_fn_ptr_basics)]
#![no_main]
#![no_std]

mod cpu;
mod io;
mod panic_wait;
mod kernel_init;

pub unsafe fn main() {
    let mut a = 0;
    loop {
        a += 1;
        if a == 10 {
            break;
        }
    }
}
