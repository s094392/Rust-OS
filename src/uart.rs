use crate::mmio::MMIO;
use core::fmt::{self, Write};

#[allow(unused_imports)]

pub fn uart_init() {
    MMIO.uart_init();
}

pub fn print(s: &str) {
    for c in s.chars() {
        MMIO.send(c);
    }
}

pub fn read_c() -> char {
    return MMIO.read();
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print(s);
        Ok(())
    }
}

struct Uart {}

#[doc(hidden)]
pub fn write_fmt(args: fmt::Arguments) {
    Uart {}.write_fmt(args).ok();
}

//#[doc(hidden)]
//pub fn write_str(s: &str) {
//Uart {}.write_str(s).ok();
//}
