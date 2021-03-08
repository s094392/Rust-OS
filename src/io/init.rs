use core::{marker::PhantomData, ops};
use register::{mmio::*, register_bitfields, register_structs};

pub struct MMIODerefWrapper<T> {
    start_addr: usize,
    phantom: PhantomData<fn() -> T>,
}

impl<T> MMIODerefWrapper<T> {
    /// Create an instance.
    pub const unsafe fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom: PhantomData,
        }
    }
}

impl<T> ops::Deref for MMIODerefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start_addr as *const _) }
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
    ]
    //AUXENB [
        //MINIUART OFFSET(0) NUMBITS(1) [
            //ENABLE = 0b1,
            //DISABLE = 0b0,
        //]
    //]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00200004 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        //(0x00215004 => AUXENB: ReadWrite<u32, GPFSEL1::Register>),
        (0x00300000 => @END),
    }
}

type Registers = MMIODerefWrapper<RegisterBlock>;

pub fn uart_init() {
    unsafe {
        let regs = Registers::new(0x3F000000);
        regs.GPFSEL1.write(GPFSEL1::FSEL14::ALT5);
        //regs.GPFSEL1.write(GPFSEL1::FSEL15::ALT5);
        regs.GPFSEL1.set(0x3);
        //regs.AUXENB.write(AUXENB::MINIUART::ENABLE);
    }
}
