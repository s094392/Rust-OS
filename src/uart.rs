use crate::mmio::UARTDriver;
pub static UART: UARTDriver = UARTDriver {
    base_addr: 0x3F000000,
};

pub fn print(s: &str) {
    for c in s.chars() {
        UART.send(c);
    }
}

pub fn read_c() -> char {
    return UART.read();
}
