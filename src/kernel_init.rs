use crate::boot::clear_bss;
use crate::uart::UART;
use crate::main;

#[no_mangle]
pub unsafe fn kernel_init() {
    clear_bss();
    UART.init();
    main();
}
