use crate::mmio::UARTDriver;
pub static UART: UARTDriver = UARTDriver {
    base_addr: 0x3F000000,
};

pub fn put_c(c: char) {
    UART.send(c);
}

pub fn read_c() -> char {
    UART.read()
}
