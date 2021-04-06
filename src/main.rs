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
use crate::frame::page_alloc;
use crate::frame::page_free;
use crate::sys::reboot;
use crate::uart::read_c;
use alloc::vec::Vec;

#[global_allocator]
static GLOBAL: PiAllocator = PiAllocator::new();

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
            } else if s == "frame" {
                println!("{}", 14);
                let mut page = page_alloc(14);
                page_free(&mut page);
            } else if s == "alloc" {
                let mut b1 = Vec::<Tmp>::new();
                b1.push(Tmp { a: 1, b: 3 });
                b1.push(Tmp { a: 1, b: 4 });
                b1.push(Tmp { a: 1, b: 5 });
                b1.push(Tmp { a: 1, b: 6 });
                b1.push(Tmp { a: 1, b: 7 });
                println!("{} {}", b1[0].a, b1[0].b);
                println!("{} {}", b1[1].a, b1[1].b);
                println!("{} {}", b1[2].a, b1[2].b);
                println!("{} {}", b1[3].a, b1[3].b);
                println!("{} {}", b1[4].a, b1[4].b);
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
