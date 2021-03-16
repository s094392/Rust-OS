use crate::mmio::MMIO;

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
