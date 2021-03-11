use core::{ops};
use register::{mmio::*, register_bitfields, register_structs};

pub struct UARTDriver {
    base_addr: usize,
}

impl UARTDriver {

    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    fn init(&self) {
        self.AUXENB.write(AUXENB::MINIUART::Disable);
        self.GPFSEL1.write(GPFSEL1::FSEL14::ALT5);
    }
}

impl ops::Deref for UARTDriver {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

register_bitfields! {
    u32,
    GPFSEL1 [
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            ALT5 = 0b010
        ],

        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            ALT5 = 0b010
        ]
    ],

    AUXENB [
        MINIUART OFFSET(0) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00200004 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x00215004 => AUXENB: ReadWrite<u32, AUXENB::Register>),
        (0x01000000 => @END),
    }
}


pub fn uart_init() {
    let uart_driver = UARTDriver{base_addr: 0x3F000000};
    uart_driver.init();
}
