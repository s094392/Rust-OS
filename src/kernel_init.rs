use crate::boot::clear_bss;
use crate::main;
use crate::uart::uart_init;

#[no_mangle]
pub unsafe fn kernel_init() {
    clear_bss();
    uart_init();
    main();
}
