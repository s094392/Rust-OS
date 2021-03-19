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
use crate::uart::print_c;
use crate::uart::read_c;

fn shell() {
    loop {
        print("$ ");
        let mut array = [0_u8; 1000];
        let mut len = 0;
        loop {
            let c = read_c();
            print_c(c);
            if c == '\n' {
                break;
            }
            array[len] = c as u8;
            len += 1;
        }
        if let Ok(s) = core::str::from_utf8(&array[..len]) {
            if s == "hello" {
                print("Hello World\r\n");
            } else if s == "help" {
                print("help: Help\r\n");
                print("reboot: Reboot\r\n");
                print("hello: Hello World\r\n");
            } else if s == "reboot" {
                reboot(3);
            } else {
                print("Commnad not found\r\n");
            }
        }
    }
}

pub fn main() {
    shell();
}
