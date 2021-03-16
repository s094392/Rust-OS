use crate::boot::clear_bss;
use crate::main;
use crate::uart::print;
use crate::uart::UART;

#[no_mangle]
pub unsafe fn kernel_init() {
    clear_bss();
    UART.init();
    print("Booting....\r\n");
    main();
}
