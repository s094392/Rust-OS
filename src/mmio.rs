use core::{ops};
use register::{mmio::*, register_bitfields, register_structs};

pub struct UARTDriver {
    pub base_addr: usize,
}

impl UARTDriver {
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    fn delay(mut cycle: u8) {
        while cycle>0 {
            cycle -=1;
        }
    }

    pub fn init(&self) {
        self.AUXENB.write(AUXENB::MINIUART::Enable);
        self.AUX_MU_CNTL.set(0);
        self.AUX_MU_IER.set(0);
        self.AUX_MU_LCR.write(AUX_MU_LCR::DATASIZE::BIT8);
        self.AUX_MU_MCR.set(0);
        self.AUX_MU_BAUD.set(270);
        self.AUX_MU_IIR.write(AUX_MU_IIR::FIFO::Disable);

        self.GPFSEL1.write(GPFSEL1::FSEL14::ALT5);
        self.GPPUD.write(GPPUD::PUD::Disable);

        UARTDriver::delay(150);
        self.GPPUDCLK0.write(GPPUDCLK0::PUDCLK14::Enable);
        self.GPPUDCLK0.write(GPPUDCLK0::PUDCLK15::Enable);
        UARTDriver::delay(150);

        self.GPPUDCLK0.set(0);
        self.AUX_MU_CNTL.set(3);
    }
}

impl ops::Deref for UARTDriver {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

register_bitfields! {
    u32, GPFSEL1 [
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

    GPPUD [
        PUD OFFSET(0) NUMBITS(2) [
            Disable = 0b00
        ]
    ],

    GPPUDCLK0 [
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ]
    ],

    AUXENB [
        MINIUART OFFSET(0) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ]
    ],

    AUX_MU_LCR [
        DATASIZE OFFSET(0) NUMBITS(2) [
            BIT7 = 0b00,
            BIT8 = 0b11
        ]
    ],

    AUX_MU_IIR [
        FIFO OFFSET(1) NUMBITS(2) [
            Disable = 0b11
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => _reserved1),
        (0x00200004 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x00200098 => GPPUDCLK0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0x0020009C => GPPUD: ReadWrite<u32, GPPUD::Register>),
        (0x00215004 => AUXENB: ReadWrite<u32, AUXENB::Register>),
        (0x00215044 => AUX_MU_IER: ReadWrite<u32>),
        (0x00215048 => AUX_MU_IIR: ReadWrite<u32, AUX_MU_IIR::Register>),
        (0x0021504C => AUX_MU_LCR: ReadWrite<u32, AUX_MU_LCR::Register>),
        (0x00215050 => AUX_MU_MCR: ReadWrite<u32>),
        (0x00215060 => AUX_MU_CNTL: ReadWrite<u32>),
        (0x00215068 => AUX_MU_BAUD: ReadWrite<u32>),
        (0x01000000 => @END),
    }
}
