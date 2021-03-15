use core::ops;
use register::{mmio::*, register_bitfields, register_structs};

pub struct UARTDriver {
    pub base_addr: usize,
}

impl UARTDriver {
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    fn delay(mut cycle: u8) {
        while cycle > 0 {
            cycle -= 1;
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
        self.GPFSEL1.write(GPFSEL1::FSEL15::ALT5);
        self.GPPUD.write(GPPUD::PUD::Disable);

        UARTDriver::delay(150);
        self.GPPUDCLK0.write(GPPUDCLK0::PUDCLK14::Enable);
        self.GPPUDCLK0.write(GPPUDCLK0::PUDCLK15::Enable);
        UARTDriver::delay(150);

        self.GPPUDCLK0.set(0);
        self.AUX_MU_CNTL.set(3);
    }

    pub fn send(&self, c: char) {
        loop {
            if self.AUX_MU_LSR.read(AUX_MU_LSR::TRANS_EMPTY) == 1 {
                break;
            }
        }
        self.AUX_MU_IO.set(c as u32);
    }

    pub fn read(&self) -> char {
        loop {
            if self.AUX_MU_LSR.read(AUX_MU_LSR::DATA_READY) == 1 {
                break;
            }
        }
        let r = self.AUX_MU_IO.get() as u8;
        if r as char == '\r' {
            '\n'
        } else {
            r as char
        }
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
    ],

    AUX_MU_LSR [
        TRANS_EMPTY OFFSET(5) NUMBITS(1) [
            Empty = 0b1,
            NotEmpty = 0b0
        ],
        DATA_READY OFFSET(0) NUMBITS(1) [
            Ready = 0b1,
            NotReady = 0b0
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => _reserved1),
        (0x00200004 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x00200008 => _reserved2),
        (0x00200094 => GPPUD: ReadWrite<u32, GPPUD::Register>),
        (0x00200098 => GPPUDCLK0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0x0020009C => _reserved3),
        (0x00215004 => AUXENB: ReadWrite<u32, AUXENB::Register>),
        (0x00215008 => _reserved4),
        (0x00215040 => AUX_MU_IO: ReadWrite<u32>),
        (0x00215044 => AUX_MU_IER: ReadWrite<u32>),
        (0x00215048 => AUX_MU_IIR: ReadWrite<u32, AUX_MU_IIR::Register>),
        (0x0021504C => AUX_MU_LCR: ReadWrite<u32, AUX_MU_LCR::Register>),
        (0x00215050 => AUX_MU_MCR: ReadWrite<u32>),
        (0x00215054 => AUX_MU_LSR: ReadWrite<u32, AUX_MU_LSR::Register>),
        (0x00215058 => _reserved5),
        (0x00215060 => AUX_MU_CNTL: ReadWrite<u32>),
        (0x00215064 => _reserved6),
        (0x00215068 => AUX_MU_BAUD: ReadWrite<u32>),
        (0x0021506C => _reserved7),
        (0x01000000 => @END),
    }
}
