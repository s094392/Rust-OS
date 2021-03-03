use crate::cpu::clear_bss;
use crate::main;

#[no_mangle]
pub unsafe fn kernel_init() {
    clear_bss();
    main();
}
