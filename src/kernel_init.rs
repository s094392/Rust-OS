use crate::boot::clear_bss;
use crate::main;
use crate::uart::uart_init;
use crate::frame::buddy_init;

#[no_mangle]
pub unsafe fn kernel_init() {
    clear_bss();
    uart_init();
    buddy_init(0x1000_0000, 0x2000_0000);
    main();
}
