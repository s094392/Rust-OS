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
#[macro_use]
mod macros;
mod uart;
use crate::sys::reboot;
use crate::uart::read_c;

fn shell() {
    loop {
        print!("$ ");
        let mut array = [0_u8; 1000];
        let mut len = 0;
        loop {
            let c = read_c();
            print!("{}", c);
            if c == '\n' {
                break;
            }
            array[len] = c as u8;
            len += 1;
        }
        if let Ok(s) = core::str::from_utf8(&array[..len]) {
            if s == "hello" {
                println!("Hello World");
            } else if s == "help" {
                println!("help: Help");
                println!("reboot: Reboot");
                println!("hello: Hello World");
            } else if s == "reboot" {
                reboot(3);
            } else {
                println!("Commnad not found");
            }
        }
    }
}

pub fn main() {
    shell();
}
