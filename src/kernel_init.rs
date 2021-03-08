use crate::cpu::clear_bss;
use crate::io::init;
use crate::main;

#[no_mangle]
pub unsafe fn kernel_init() {
    clear_bss();
    init::uart_init();
    main();
}
