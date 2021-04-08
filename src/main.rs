#![feature(asm)]
#![feature(global_asm)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]
#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod allocator;
mod boot;
mod frame;
mod kernel_init;
mod mmio;
mod panic_wait;
mod sys;
mod uart;
extern crate alloc;
use crate::allocator::PiAllocator;
use crate::sys::reboot;
use crate::uart::read_c;
use alloc::vec::Vec;

#[global_allocator]
static mut GLOBAL: PiAllocator = PiAllocator::new();

struct Tmp {
    a: u32,
    b: u32,
}

fn shell() {
    loop {
        print!("$ ");
        let mut array = [0_u8; 1000];
        let mut len = 0;
        loop {
            let c = read_c();
            if c == '\n' {
                println!("");
                break;
            }
            print!("{}", c);
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
            } else if s == "alloc" {
                for _i in 0..1 {
                    let mut b1 = Vec::<Tmp>::with_capacity(1);
                    b1.push(Tmp { a: 1, b: 3 });
                    println!("{} {}", b1[0].a, b1[0].b);
                }
            } else {
                println!("Commnad not found");
                match s.parse::<u32>() {
                    Ok(i) => println!("your integer input: {}", i),
                    Err(..) => println!("this was not an integer: {}", s),
                };
            }
        }
    }
}

pub fn main() {
    shell();
}
