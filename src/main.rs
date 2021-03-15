#![feature(asm)]
#![feature(global_asm)]
#![feature(const_fn_fn_ptr_basics)]
#![no_main]
#![no_std]

mod boot;
mod kernel_init;
mod mmio;
mod panic_wait;
mod uart;
use crate::uart::put_c;
use crate::uart::read_c;

pub unsafe fn main() {
    loop {
        let c = read_c();
        put_c(c);
    }
}
