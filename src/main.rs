#![feature(asm)]
#![feature(global_asm)]
#![feature(const_fn_fn_ptr_basics)]
#![no_main]
#![no_std]

mod boot;
mod kernel_init;
mod mmio;
mod panic_wait;
mod sys;
mod uart;
use crate::sys::reboot;
use crate::uart::print;
use crate::uart::read_c;

pub unsafe fn main() {
    print("Hello world\r\n");
    let _c = read_c();
    reboot(2);
}
