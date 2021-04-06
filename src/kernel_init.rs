use crate::main;
use crate::uart::uart_init;
use crate::{boot::clear_bss, GLOBAL};

#[no_mangle]
pub unsafe fn kernel_init() {
    clear_bss();
    uart_init();
    GLOBAL.init();
    main();
}
