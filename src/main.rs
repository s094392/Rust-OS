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

fn shell() {
    loop {
        print("$ ");
        let c = read_c();
        print("\r\n");
        if c == 'r' {
            reboot(3);
        } else if c == 'p' {
            print("Hello World\r\n");
        } else if c == 'h' {
            print("h: Help\r\n");
            print("r: Reboot\r\n");
            print("p: Hello World\r\n");
        }
    }
}

pub fn main() {
    shell();
}
