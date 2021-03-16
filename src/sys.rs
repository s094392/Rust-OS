use crate::mmio::MMIO;

pub fn reboot(tick: u32) {
    MMIO.reboot(tick);
}
