use crate::boot::clear_bss;
use crate::main;
use crate::uart::put_c;
use crate::uart::UART;

#[no_mangle]
pub unsafe fn kernel_init() {
    clear_bss();
    UART.init();
    put_c('D');
    put_c('o');
    put_c('n');
    put_c('e');
    put_c('\r');
    put_c('\n');
    main();
}
