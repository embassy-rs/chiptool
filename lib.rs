#![no_std]
#![doc = "Peripheral access API (generated using svd2rust v0.1.0 (a0b25eb 2021-05-23))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD"]
    PVD = 1,
    #[doc = "2 - TAMP_STAMP"]
    TAMP_STAMP = 2,
    #[doc = "3 - RTC_WKUP"]
    RTC_WKUP = 3,
    #[doc = "4 - FLASH"]
    FLASH = 4,
    #[doc = "5 - RCC"]
    RCC = 5,
    #[doc = "6 - EXTI0"]
    EXTI0 = 6,
    #[doc = "7 - EXTI1"]
    EXTI1 = 7,
    #[doc = "8 - EXTI2"]
    EXTI2 = 8,
    #[doc = "9 - EXTI3"]
    EXTI3 = 9,
    #[doc = "10 - EXTI4"]
    EXTI4 = 10,
    #[doc = "11 - DMA1_STREAM0"]
    DMA1_STREAM0 = 11,
    #[doc = "12 - DMA1_STREAM1"]
    DMA1_STREAM1 = 12,
    #[doc = "13 - DMA1_STREAM2"]
    DMA1_STREAM2 = 13,
    #[doc = "14 - DMA1_STREAM3"]
    DMA1_STREAM3 = 14,
    #[doc = "15 - DMA1_STREAM4"]
    DMA1_STREAM4 = 15,
    #[doc = "16 - DMA1_STREAM5"]
    DMA1_STREAM5 = 16,
    #[doc = "17 - DMA1_STREAM6"]
    DMA1_STREAM6 = 17,
    #[doc = "18 - ADC"]
    ADC = 18,
    #[doc = "19 - CAN1_TX"]
    CAN1_TX = 19,
    #[doc = "20 - CAN1_RX0"]
    CAN1_RX0 = 20,
    #[doc = "21 - CAN1_RX1"]
    CAN1_RX1 = 21,
    #[doc = "22 - CAN1_SCE"]
    CAN1_SCE = 22,
    #[doc = "23 - EXTI9_5"]
    EXTI9_5 = 23,
    #[doc = "24 - TIM1_BRK_TIM9"]
    TIM1_BRK_TIM9 = 24,
    #[doc = "25 - TIM1_UP_TIM10"]
    TIM1_UP_TIM10 = 25,
    #[doc = "26 - TIM1_TRG_COM_TIM11"]
    TIM1_TRG_COM_TIM11 = 26,
    #[doc = "27 - TIM1_CC"]
    TIM1_CC = 27,
    #[doc = "28 - TIM2"]
    TIM2 = 28,
    #[doc = "29 - TIM3"]
    TIM3 = 29,
    #[doc = "30 - TIM4"]
    TIM4 = 30,
    #[doc = "31 - I2C1_EV"]
    I2C1_EV = 31,
    #[doc = "32 - I2C1_ER"]
    I2C1_ER = 32,
    #[doc = "33 - I2C2_EV"]
    I2C2_EV = 33,
    #[doc = "34 - I2C2_ER"]
    I2C2_ER = 34,
    #[doc = "35 - SPI1"]
    SPI1 = 35,
    #[doc = "36 - SPI2"]
    SPI2 = 36,
    #[doc = "37 - USART1"]
    USART1 = 37,
    #[doc = "38 - USART2"]
    USART2 = 38,
    #[doc = "39 - USART3"]
    USART3 = 39,
    #[doc = "40 - EXTI15_10"]
    EXTI15_10 = 40,
    #[doc = "41 - RTC_ALARM"]
    RTC_ALARM = 41,
    #[doc = "42 - OTG_FS_WKUP"]
    OTG_FS_WKUP = 42,
    #[doc = "43 - TIM8_BRK_TIM12"]
    TIM8_BRK_TIM12 = 43,
    #[doc = "44 - TIM8_UP_TIM13"]
    TIM8_UP_TIM13 = 44,
    #[doc = "45 - TIM8_TRG_COM_TIM14"]
    TIM8_TRG_COM_TIM14 = 45,
    #[doc = "46 - TIM8_CC"]
    TIM8_CC = 46,
    #[doc = "47 - DMA1_STREAM7"]
    DMA1_STREAM7 = 47,
    #[doc = "48 - FMC"]
    FMC = 48,
    #[doc = "49 - SDIO"]
    SDIO = 49,
    #[doc = "50 - TIM5"]
    TIM5 = 50,
    #[doc = "51 - SPI3"]
    SPI3 = 51,
    #[doc = "52 - UART4"]
    UART4 = 52,
    #[doc = "53 - UART5"]
    UART5 = 53,
    #[doc = "54 - TIM6_DAC"]
    TIM6_DAC = 54,
    #[doc = "55 - TIM7"]
    TIM7 = 55,
    #[doc = "56 - DMA2_STREAM0"]
    DMA2_STREAM0 = 56,
    #[doc = "57 - DMA2_STREAM1"]
    DMA2_STREAM1 = 57,
    #[doc = "58 - DMA2_STREAM2"]
    DMA2_STREAM2 = 58,
    #[doc = "59 - DMA2_STREAM3"]
    DMA2_STREAM3 = 59,
    #[doc = "60 - DMA2_STREAM4"]
    DMA2_STREAM4 = 60,
    #[doc = "61 - ETH"]
    ETH = 61,
    #[doc = "62 - ETH_WKUP"]
    ETH_WKUP = 62,
    #[doc = "63 - CAN2_TX"]
    CAN2_TX = 63,
    #[doc = "64 - CAN2_RX0"]
    CAN2_RX0 = 64,
    #[doc = "65 - CAN2_RX1"]
    CAN2_RX1 = 65,
    #[doc = "66 - CAN2_SCE"]
    CAN2_SCE = 66,
    #[doc = "67 - OTG_FS"]
    OTG_FS = 67,
    #[doc = "68 - DMA2_STREAM5"]
    DMA2_STREAM5 = 68,
    #[doc = "69 - DMA2_STREAM6"]
    DMA2_STREAM6 = 69,
    #[doc = "70 - DMA2_STREAM7"]
    DMA2_STREAM7 = 70,
    #[doc = "71 - USART6"]
    USART6 = 71,
    #[doc = "72 - I2C3_EV"]
    I2C3_EV = 72,
    #[doc = "73 - I2C3_ER"]
    I2C3_ER = 73,
    #[doc = "74 - OTG_HS_EP1_OUT"]
    OTG_HS_EP1_OUT = 74,
    #[doc = "75 - OTG_HS_EP1_IN"]
    OTG_HS_EP1_IN = 75,
    #[doc = "76 - OTG_HS_WKUP"]
    OTG_HS_WKUP = 76,
    #[doc = "77 - OTG_HS"]
    OTG_HS = 77,
    #[doc = "78 - DCMI"]
    DCMI = 78,
    #[doc = "80 - HASH_RNG"]
    HASH_RNG = 80,
    #[doc = "81 - FPU"]
    FPU = 81,
    #[doc = "82 - UART7"]
    UART7 = 82,
    #[doc = "83 - UART8"]
    UART8 = 83,
    #[doc = "84 - SPI4"]
    SPI4 = 84,
    #[doc = "85 - SPI5"]
    SPI5 = 85,
    #[doc = "86 - SPI6"]
    SPI6 = 86,
    #[doc = "87 - SAI1"]
    SAI1 = 87,
    #[doc = "88 - LTDC"]
    LTDC = 88,
    #[doc = "89 - LTDC_ER"]
    LTDC_ER = 89,
    #[doc = "90 - DMA2D"]
    DMA2D = 90,
}
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_STREAM0();
    fn DMA1_STREAM1();
    fn DMA1_STREAM2();
    fn DMA1_STREAM3();
    fn DMA1_STREAM4();
    fn DMA1_STREAM5();
    fn DMA1_STREAM6();
    fn ADC();
    fn CAN1_TX();
    fn CAN1_RX0();
    fn CAN1_RX1();
    fn CAN1_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM9();
    fn TIM1_UP_TIM10();
    fn TIM1_TRG_COM_TIM11();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn OTG_FS_WKUP();
    fn TIM8_BRK_TIM12();
    fn TIM8_UP_TIM13();
    fn TIM8_TRG_COM_TIM14();
    fn TIM8_CC();
    fn DMA1_STREAM7();
    fn FMC();
    fn SDIO();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DAC();
    fn TIM7();
    fn DMA2_STREAM0();
    fn DMA2_STREAM1();
    fn DMA2_STREAM2();
    fn DMA2_STREAM3();
    fn DMA2_STREAM4();
    fn ETH();
    fn ETH_WKUP();
    fn CAN2_TX();
    fn CAN2_RX0();
    fn CAN2_RX1();
    fn CAN2_SCE();
    fn OTG_FS();
    fn DMA2_STREAM5();
    fn DMA2_STREAM6();
    fn DMA2_STREAM7();
    fn USART6();
    fn I2C3_EV();
    fn I2C3_ER();
    fn OTG_HS_EP1_OUT();
    fn OTG_HS_EP1_IN();
    fn OTG_HS_WKUP();
    fn OTG_HS();
    fn DCMI();
    fn HASH_RNG();
    fn FPU();
    fn UART7();
    fn UART8();
    fn SPI4();
    fn SPI5();
    fn SPI6();
    fn SAI1();
    fn LTDC();
    fn LTDC_ER();
    fn DMA2D();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 91] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector {
        _handler: TAMP_STAMP,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_STREAM0,
    },
    Vector {
        _handler: DMA1_STREAM1,
    },
    Vector {
        _handler: DMA1_STREAM2,
    },
    Vector {
        _handler: DMA1_STREAM3,
    },
    Vector {
        _handler: DMA1_STREAM4,
    },
    Vector {
        _handler: DMA1_STREAM5,
    },
    Vector {
        _handler: DMA1_STREAM6,
    },
    Vector { _handler: ADC },
    Vector { _handler: CAN1_TX },
    Vector { _handler: CAN1_RX0 },
    Vector { _handler: CAN1_RX1 },
    Vector { _handler: CAN1_SCE },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM9,
    },
    Vector {
        _handler: TIM1_UP_TIM10,
    },
    Vector {
        _handler: TIM1_TRG_COM_TIM11,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector {
        _handler: OTG_FS_WKUP,
    },
    Vector {
        _handler: TIM8_BRK_TIM12,
    },
    Vector {
        _handler: TIM8_UP_TIM13,
    },
    Vector {
        _handler: TIM8_TRG_COM_TIM14,
    },
    Vector { _handler: TIM8_CC },
    Vector {
        _handler: DMA1_STREAM7,
    },
    Vector { _handler: FMC },
    Vector { _handler: SDIO },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: TIM6_DAC },
    Vector { _handler: TIM7 },
    Vector {
        _handler: DMA2_STREAM0,
    },
    Vector {
        _handler: DMA2_STREAM1,
    },
    Vector {
        _handler: DMA2_STREAM2,
    },
    Vector {
        _handler: DMA2_STREAM3,
    },
    Vector {
        _handler: DMA2_STREAM4,
    },
    Vector { _handler: ETH },
    Vector { _handler: ETH_WKUP },
    Vector { _handler: CAN2_TX },
    Vector { _handler: CAN2_RX0 },
    Vector { _handler: CAN2_RX1 },
    Vector { _handler: CAN2_SCE },
    Vector { _handler: OTG_FS },
    Vector {
        _handler: DMA2_STREAM5,
    },
    Vector {
        _handler: DMA2_STREAM6,
    },
    Vector {
        _handler: DMA2_STREAM7,
    },
    Vector { _handler: USART6 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector {
        _handler: OTG_HS_EP1_OUT,
    },
    Vector {
        _handler: OTG_HS_EP1_IN,
    },
    Vector {
        _handler: OTG_HS_WKUP,
    },
    Vector { _handler: OTG_HS },
    Vector { _handler: DCMI },
    Vector { _reserved: 0 },
    Vector { _handler: HASH_RNG },
    Vector { _handler: FPU },
    Vector { _handler: UART7 },
    Vector { _handler: UART8 },
    Vector { _handler: SPI4 },
    Vector { _handler: SPI5 },
    Vector { _handler: SPI6 },
    Vector { _handler: SAI1 },
    Vector { _handler: LTDC },
    Vector { _handler: LTDC_ER },
    Vector { _handler: DMA2D },
];
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
pub const SAI1: *mut () = 0x4001_5800 as u32 as _;
pub const SPI6: spi::Spi = spi::Spi(0x4001_5400 as u32 as _);
pub const TIM10: *mut () = 0x4001_4400 as u32 as _;
pub const TIM5: *mut () = 0x4000_0c00 as u32 as _;
pub const SDIO: *mut () = 0x4001_2c00 as u32 as _;
pub const TIM4: *mut () = 0x4000_0800 as u32 as _;
pub const TIM9: *mut () = 0x4001_4000 as u32 as _;
pub const CAN2: *mut () = 0x4000_6800 as u32 as _;
pub const DCMI: *mut () = 0x5005_0000 as u32 as _;
pub const SPI4: spi::Spi = spi::Spi(0x4001_3400 as u32 as _);
pub const GPIOH: gpio::Gpio = gpio::Gpio(0x4002_1c00 as u32 as _);
pub const GPIOK: gpio::Gpio = gpio::Gpio(0x4002_2800 as u32 as _);
pub const SYSCFG: syscfg::Syscfg = syscfg::Syscfg(0x4001_3800 as u32 as _);
pub const IWDG: *mut () = 0x4000_3000 as u32 as _;
pub const TIM6: *mut () = 0x4000_1000 as u32 as _;
pub const RTC: *mut () = 0x4000_2800 as u32 as _;
pub const UART5: *mut () = 0x4000_5000 as u32 as _;
pub const USART6: usart::Usart = usart::Usart(0x4001_1400 as u32 as _);
pub const GPIOB: gpio::Gpio = gpio::Gpio(0x4002_0400 as u32 as _);
pub const TIM14: *mut () = 0x4000_2000 as u32 as _;
pub const USART3: usart::Usart = usart::Usart(0x4000_4800 as u32 as _);
pub const SPI3: spi::Spi = spi::Spi(0x4000_3c00 as u32 as _);
pub const GPIOG: gpio::Gpio = gpio::Gpio(0x4002_1800 as u32 as _);
pub const GPIOJ: gpio::Gpio = gpio::Gpio(0x4002_2400 as u32 as _);
pub const LTDC: *mut () = 0x4001_6800 as u32 as _;
pub const TIM7: *mut () = 0x4000_1400 as u32 as _;
pub const GPIOF: gpio::Gpio = gpio::Gpio(0x4002_1400 as u32 as _);
pub const TIM13: *mut () = 0x4000_1c00 as u32 as _;
pub const USB_OTG_HS: *mut () = 0x4004_0000 as u32 as _;
pub const USB_OTG_FS: *mut () = 0x5000_0000 as u32 as _;
pub const DAC: *mut () = 0x4000_7400 as u32 as _;
pub const I2C3: *mut () = 0x4000_5c00 as u32 as _;
pub const SPI5: spi::Spi = spi::Spi(0x4001_5000 as u32 as _);
pub const TIM11: *mut () = 0x4001_4800 as u32 as _;
pub const DMA2: dma::Dma = dma::Dma(0x4002_6400 as u32 as _);
pub const I2C1: *mut () = 0x4000_5400 as u32 as _;
pub const SPI1: spi::Spi = spi::Spi(0x4001_3000 as u32 as _);
pub const ADC2: *mut () = 0x4001_2100 as u32 as _;
pub const EXTI: exti::Exti = exti::Exti(0x4001_3c00 as u32 as _);
pub const I2C2: *mut () = 0x4000_5800 as u32 as _;
pub const USART1: usart::Usart = usart::Usart(0x4001_1000 as u32 as _);
pub const GPIOE: gpio::Gpio = gpio::Gpio(0x4002_1000 as u32 as _);
pub const GPIOD: gpio::Gpio = gpio::Gpio(0x4002_0c00 as u32 as _);
pub const TIM12: *mut () = 0x4000_1800 as u32 as _;
pub const ETH: *mut () = 0x4002_8000 as u32 as _;
pub const ADC1: *mut () = 0x4001_2000 as u32 as _;
pub const WWDG: *mut () = 0x4000_2c00 as u32 as _;
pub const CAN1: *mut () = 0x4000_6400 as u32 as _;
pub const TIM2: *mut () = 0x4000_0000 as u32 as _;
pub const TIM3: *mut () = 0x4000_0400 as u32 as _;
pub const GPIOC: gpio::Gpio = gpio::Gpio(0x4002_0800 as u32 as _);
pub const RCC: *mut () = 0x4002_3800 as u32 as _;
pub const TIM1: *mut () = 0x4001_0000 as u32 as _;
pub const DMA2D: *mut () = 0x4002_b000 as u32 as _;
pub const UART8: *mut () = 0x4000_7c00 as u32 as _;
pub const FLASH: *mut () = 0x4002_3c00 as u32 as _;
pub const USART2: usart::Usart = usart::Usart(0x4000_4400 as u32 as _);
pub const RNG: rng::Rng = rng::Rng(0x5006_0800 as u32 as _);
pub const GPIOI: gpio::Gpio = gpio::Gpio(0x4002_2000 as u32 as _);
pub const DMA1: dma::Dma = dma::Dma(0x4002_6000 as u32 as _);
pub const UART7: *mut () = 0x4000_7800 as u32 as _;
pub const UART4: *mut () = 0x4000_4c00 as u32 as _;
pub const ADC3: *mut () = 0x4001_2200 as u32 as _;
pub const TIM8: *mut () = 0x4001_0400 as u32 as _;
pub const SPI2: spi::Spi = spi::Spi(0x4000_3800 as u32 as _);
pub const GPIOA: gpio::Gpio = gpio::Gpio(0x4002_0000 as u32 as _);
pub const DBGMCU: *mut () = 0xe004_2000 as u32 as _;
pub mod usart {
    use crate::generic::*;
    #[doc = "Universal asynchronous receiver transmitter"]
    #[derive(Copy, Clone)]
    pub struct Uart(pub *mut u8);
    unsafe impl Send for Uart {}
    unsafe impl Sync for Uart {}
    impl Uart {
        #[doc = "Status register"]
        pub fn sr(self) -> Reg<regs::Sr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Data register"]
        pub fn dr(self) -> Reg<regs::Dr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Baud rate register"]
        pub fn brr(self) -> Reg<regs::Brr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Control register 3"]
        pub fn cr3(self) -> Reg<regs::Cr3, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
    }
    #[doc = "Universal synchronous asynchronous receiver transmitter"]
    #[derive(Copy, Clone)]
    pub struct Usart(pub *mut u8);
    unsafe impl Send for Usart {}
    unsafe impl Sync for Usart {}
    impl Usart {
        #[doc = "Status register"]
        pub fn sr(self) -> Reg<regs::Sr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Data register"]
        pub fn dr(self) -> Reg<regs::Dr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Baud rate register"]
        pub fn brr(self) -> Reg<regs::Brr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2Usart, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Control register 3"]
        pub fn cr3(self) -> Reg<regs::Cr3Usart, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Guard time and prescaler register"]
        pub fn gtpr(self) -> Reg<regs::Gtpr, RW> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "Control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr1(pub u32);
        impl Cr1 {
            #[doc = "Send break"]
            pub const fn sbk(&self) -> super::vals::Sbk {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Sbk(val as u8)
            }
            #[doc = "Send break"]
            pub fn set_sbk(&mut self, val: super::vals::Sbk) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "Receiver wakeup"]
            pub const fn rwu(&self) -> super::vals::Rwu {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Rwu(val as u8)
            }
            #[doc = "Receiver wakeup"]
            pub fn set_rwu(&mut self, val: super::vals::Rwu) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "Receiver enable"]
            pub const fn re(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Receiver enable"]
            pub fn set_re(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Transmitter enable"]
            pub const fn te(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Transmitter enable"]
            pub fn set_te(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "IDLE interrupt enable"]
            pub const fn idleie(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "IDLE interrupt enable"]
            pub fn set_idleie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "RXNE interrupt enable"]
            pub const fn rxneie(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "RXNE interrupt enable"]
            pub fn set_rxneie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Transmission complete interrupt enable"]
            pub const fn tcie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transmission complete interrupt enable"]
            pub fn set_tcie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "TXE interrupt enable"]
            pub const fn txeie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "TXE interrupt enable"]
            pub fn set_txeie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "PE interrupt enable"]
            pub const fn peie(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "PE interrupt enable"]
            pub fn set_peie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Parity selection"]
            pub const fn ps(&self) -> super::vals::Ps {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Ps(val as u8)
            }
            #[doc = "Parity selection"]
            pub fn set_ps(&mut self, val: super::vals::Ps) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "Parity control enable"]
            pub const fn pce(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "Parity control enable"]
            pub fn set_pce(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "Wakeup method"]
            pub const fn wake(&self) -> super::vals::Wake {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Wake(val as u8)
            }
            #[doc = "Wakeup method"]
            pub fn set_wake(&mut self, val: super::vals::Wake) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "Word length"]
            pub const fn m(&self) -> super::vals::M {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::M(val as u8)
            }
            #[doc = "Word length"]
            pub fn set_m(&mut self, val: super::vals::M) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "USART enable"]
            pub const fn ue(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "USART enable"]
            pub fn set_ue(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Cr1 {
            fn default() -> Cr1 {
                Cr1(0)
            }
        }
        #[doc = "Control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2Usart(pub u32);
        impl Cr2Usart {
            #[doc = "Address of the USART node"]
            pub const fn add(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Address of the USART node"]
            pub fn set_add(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "lin break detection length"]
            pub const fn lbdl(&self) -> super::vals::Lbdl {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Lbdl(val as u8)
            }
            #[doc = "lin break detection length"]
            pub fn set_lbdl(&mut self, val: super::vals::Lbdl) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "LIN break detection interrupt enable"]
            pub const fn lbdie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "LIN break detection interrupt enable"]
            pub fn set_lbdie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Last bit clock pulse"]
            pub const fn lbcl(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Last bit clock pulse"]
            pub fn set_lbcl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Clock phase"]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Cpha(val as u8)
            }
            #[doc = "Clock phase"]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "Clock polarity"]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Cpol(val as u8)
            }
            #[doc = "Clock polarity"]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "Clock enable"]
            pub const fn clken(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "Clock enable"]
            pub fn set_clken(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "STOP bits"]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 12usize) & 0x03;
                super::vals::Stop(val as u8)
            }
            #[doc = "STOP bits"]
            pub fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x03 << 12usize)) | (((val.0 as u32) & 0x03) << 12usize);
            }
            #[doc = "LIN mode enable"]
            pub const fn linen(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "LIN mode enable"]
            pub fn set_linen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
        }
        impl Default for Cr2Usart {
            fn default() -> Cr2Usart {
                Cr2Usart(0)
            }
        }
        #[doc = "Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SrUsart(pub u32);
        impl SrUsart {
            #[doc = "Parity error"]
            pub const fn pe(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            pub fn set_pe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Framing error"]
            pub const fn fe(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error"]
            pub fn set_fe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Noise error flag"]
            pub const fn ne(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Noise error flag"]
            pub fn set_ne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Overrun error"]
            pub const fn ore(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            pub fn set_ore(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "IDLE line detected"]
            pub const fn idle(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "IDLE line detected"]
            pub fn set_idle(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Read data register not empty"]
            pub const fn rxne(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Read data register not empty"]
            pub fn set_rxne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Transmission complete"]
            pub const fn tc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transmission complete"]
            pub fn set_tc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Transmit data register empty"]
            pub const fn txe(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit data register empty"]
            pub fn set_txe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "LIN break detection flag"]
            pub const fn lbd(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "LIN break detection flag"]
            pub fn set_lbd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "CTS flag"]
            pub const fn cts(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "CTS flag"]
            pub fn set_cts(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
        }
        impl Default for SrUsart {
            fn default() -> SrUsart {
                SrUsart(0)
            }
        }
        #[doc = "Control register 3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr3(pub u32);
        impl Cr3 {
            #[doc = "Error interrupt enable"]
            pub const fn eie(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Error interrupt enable"]
            pub fn set_eie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "IrDA mode enable"]
            pub const fn iren(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "IrDA mode enable"]
            pub fn set_iren(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "IrDA low-power"]
            pub const fn irlp(&self) -> super::vals::Irlp {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Irlp(val as u8)
            }
            #[doc = "IrDA low-power"]
            pub fn set_irlp(&mut self, val: super::vals::Irlp) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "Half-duplex selection"]
            pub const fn hdsel(&self) -> super::vals::Hdsel {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Hdsel(val as u8)
            }
            #[doc = "Half-duplex selection"]
            pub fn set_hdsel(&mut self, val: super::vals::Hdsel) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "DMA enable receiver"]
            pub const fn dmar(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "DMA enable receiver"]
            pub fn set_dmar(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "DMA enable transmitter"]
            pub const fn dmat(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "DMA enable transmitter"]
            pub fn set_dmat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Cr3 {
            fn default() -> Cr3 {
                Cr3(0)
            }
        }
        #[doc = "Data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dr(pub u32);
        impl Dr {
            #[doc = "Data value"]
            pub const fn dr(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0x01ff;
                val as u16
            }
            #[doc = "Data value"]
            pub fn set_dr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
            }
        }
        impl Default for Dr {
            fn default() -> Dr {
                Dr(0)
            }
        }
        #[doc = "Status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sr(pub u32);
        impl Sr {
            #[doc = "Parity error"]
            pub const fn pe(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Parity error"]
            pub fn set_pe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Framing error"]
            pub const fn fe(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Framing error"]
            pub fn set_fe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Noise error flag"]
            pub const fn ne(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Noise error flag"]
            pub fn set_ne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Overrun error"]
            pub const fn ore(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun error"]
            pub fn set_ore(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "IDLE line detected"]
            pub const fn idle(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "IDLE line detected"]
            pub fn set_idle(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Read data register not empty"]
            pub const fn rxne(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Read data register not empty"]
            pub fn set_rxne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Transmission complete"]
            pub const fn tc(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Transmission complete"]
            pub fn set_tc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Transmit data register empty"]
            pub const fn txe(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit data register empty"]
            pub fn set_txe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "LIN break detection flag"]
            pub const fn lbd(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "LIN break detection flag"]
            pub fn set_lbd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Sr {
            fn default() -> Sr {
                Sr(0)
            }
        }
        #[doc = "Control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2(pub u32);
        impl Cr2 {
            #[doc = "Address of the USART node"]
            pub const fn add(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "Address of the USART node"]
            pub fn set_add(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "lin break detection length"]
            pub const fn lbdl(&self) -> super::vals::Lbdl {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Lbdl(val as u8)
            }
            #[doc = "lin break detection length"]
            pub fn set_lbdl(&mut self, val: super::vals::Lbdl) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "LIN break detection interrupt enable"]
            pub const fn lbdie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "LIN break detection interrupt enable"]
            pub fn set_lbdie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "STOP bits"]
            pub const fn stop(&self) -> super::vals::Stop {
                let val = (self.0 >> 12usize) & 0x03;
                super::vals::Stop(val as u8)
            }
            #[doc = "STOP bits"]
            pub fn set_stop(&mut self, val: super::vals::Stop) {
                self.0 = (self.0 & !(0x03 << 12usize)) | (((val.0 as u32) & 0x03) << 12usize);
            }
            #[doc = "LIN mode enable"]
            pub const fn linen(&self) -> bool {
                let val = (self.0 >> 14usize) & 0x01;
                val != 0
            }
            #[doc = "LIN mode enable"]
            pub fn set_linen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
            }
        }
        impl Default for Cr2 {
            fn default() -> Cr2 {
                Cr2(0)
            }
        }
        #[doc = "Baud rate register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Brr(pub u32);
        impl Brr {
            #[doc = "fraction of USARTDIV"]
            pub const fn div_fraction(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x0f;
                val as u8
            }
            #[doc = "fraction of USARTDIV"]
            pub fn set_div_fraction(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
            }
            #[doc = "mantissa of USARTDIV"]
            pub const fn div_mantissa(&self) -> u16 {
                let val = (self.0 >> 4usize) & 0x0fff;
                val as u16
            }
            #[doc = "mantissa of USARTDIV"]
            pub fn set_div_mantissa(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
            }
        }
        impl Default for Brr {
            fn default() -> Brr {
                Brr(0)
            }
        }
        #[doc = "Guard time and prescaler register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gtpr(pub u32);
        impl Gtpr {
            #[doc = "Prescaler value"]
            pub const fn psc(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0xff;
                val as u8
            }
            #[doc = "Prescaler value"]
            pub fn set_psc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
            }
            #[doc = "Guard time value"]
            pub const fn gt(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0xff;
                val as u8
            }
            #[doc = "Guard time value"]
            pub fn set_gt(&mut self, val: u8) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
            }
        }
        impl Default for Gtpr {
            fn default() -> Gtpr {
                Gtpr(0)
            }
        }
        #[doc = "Control register 3"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr3Usart(pub u32);
        impl Cr3Usart {
            #[doc = "Error interrupt enable"]
            pub const fn eie(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Error interrupt enable"]
            pub fn set_eie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "IrDA mode enable"]
            pub const fn iren(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "IrDA mode enable"]
            pub fn set_iren(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "IrDA low-power"]
            pub const fn irlp(&self) -> super::vals::Irlp {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Irlp(val as u8)
            }
            #[doc = "IrDA low-power"]
            pub fn set_irlp(&mut self, val: super::vals::Irlp) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "Half-duplex selection"]
            pub const fn hdsel(&self) -> super::vals::Hdsel {
                let val = (self.0 >> 3usize) & 0x01;
                super::vals::Hdsel(val as u8)
            }
            #[doc = "Half-duplex selection"]
            pub fn set_hdsel(&mut self, val: super::vals::Hdsel) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
            }
            #[doc = "Smartcard NACK enable"]
            pub const fn nack(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Smartcard NACK enable"]
            pub fn set_nack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Smartcard mode enable"]
            pub const fn scen(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Smartcard mode enable"]
            pub fn set_scen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "DMA enable receiver"]
            pub const fn dmar(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "DMA enable receiver"]
            pub fn set_dmar(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "DMA enable transmitter"]
            pub const fn dmat(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "DMA enable transmitter"]
            pub fn set_dmat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "RTS enable"]
            pub const fn rtse(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "RTS enable"]
            pub fn set_rtse(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "CTS enable"]
            pub const fn ctse(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "CTS enable"]
            pub fn set_ctse(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "CTS interrupt enable"]
            pub const fn ctsie(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "CTS interrupt enable"]
            pub fn set_ctsie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
        }
        impl Default for Cr3Usart {
            fn default() -> Cr3Usart {
                Cr3Usart(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpol(pub u8);
        impl Cpol {
            #[doc = "Steady low value on CK pin outside transmission window"]
            pub const LOW: Self = Self(0);
            #[doc = "Steady high value on CK pin outside transmission window"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct M(pub u8);
        impl M {
            #[doc = "8 data bits"]
            pub const M8: Self = Self(0);
            #[doc = "9 data bits"]
            pub const M9: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Wake(pub u8);
        impl Wake {
            #[doc = "USART wakeup on idle line"]
            pub const IDLELINE: Self = Self(0);
            #[doc = "USART wakeup on address mark"]
            pub const ADDRESSMARK: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Sbk(pub u8);
        impl Sbk {
            #[doc = "No break character is transmitted"]
            pub const NOBREAK: Self = Self(0);
            #[doc = "Break character transmitted"]
            pub const BREAK: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ps(pub u8);
        impl Ps {
            #[doc = "Even parity"]
            pub const EVEN: Self = Self(0);
            #[doc = "Odd parity"]
            pub const ODD: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpha(pub u8);
        impl Cpha {
            #[doc = "The first clock transition is the first data capture edge"]
            pub const FIRST: Self = Self(0);
            #[doc = "The second clock transition is the first data capture edge"]
            pub const SECOND: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Stop(pub u8);
        impl Stop {
            #[doc = "1 stop bit"]
            pub const STOP1: Self = Self(0);
            #[doc = "0.5 stop bits"]
            pub const STOP0P5: Self = Self(0x01);
            #[doc = "2 stop bits"]
            pub const STOP2: Self = Self(0x02);
            #[doc = "1.5 stop bits"]
            pub const STOP1P5: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hdsel(pub u8);
        impl Hdsel {
            #[doc = "Half duplex mode is not selected"]
            pub const FULLDUPLEX: Self = Self(0);
            #[doc = "Half duplex mode is selected"]
            pub const HALFDUPLEX: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rwu(pub u8);
        impl Rwu {
            #[doc = "Receiver in active mode"]
            pub const ACTIVE: Self = Self(0);
            #[doc = "Receiver in mute mode"]
            pub const MUTE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lbdl(pub u8);
        impl Lbdl {
            #[doc = "10-bit break detection"]
            pub const LBDL10: Self = Self(0);
            #[doc = "11-bit break detection"]
            pub const LBDL11: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Irlp(pub u8);
        impl Irlp {
            #[doc = "Normal mode"]
            pub const NORMAL: Self = Self(0);
            #[doc = "Low-power mode"]
            pub const LOWPOWER: Self = Self(0x01);
        }
    }
}
pub mod exti {
    use crate::generic::*;
    #[doc = "External interrupt/event controller"]
    #[derive(Copy, Clone)]
    pub struct Exti(pub *mut u8);
    unsafe impl Send for Exti {}
    unsafe impl Sync for Exti {}
    impl Exti {
        #[doc = "Interrupt mask register (EXTI_IMR)"]
        pub fn imr(self) -> Reg<regs::Imr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Event mask register (EXTI_EMR)"]
        pub fn emr(self) -> Reg<regs::Emr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Rising Trigger selection register (EXTI_RTSR)"]
        pub fn rtsr(self) -> Reg<regs::Rtsr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Falling Trigger selection register (EXTI_FTSR)"]
        pub fn ftsr(self) -> Reg<regs::Ftsr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Software interrupt event register (EXTI_SWIER)"]
        pub fn swier(self) -> Reg<regs::Swier, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Pending register (EXTI_PR)"]
        pub fn pr(self) -> Reg<regs::Pr, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "Pending register (EXTI_PR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pr(pub u32);
        impl Pr {
            #[doc = "Pending bit 0"]
            pub fn pr(&self, n: usize) -> bool {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Pending bit 0"]
            pub fn set_pr(&mut self, n: usize, val: bool) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Pr {
            fn default() -> Pr {
                Pr(0)
            }
        }
        #[doc = "Rising Trigger selection register (EXTI_RTSR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rtsr(pub u32);
        impl Rtsr {
            #[doc = "Rising trigger event configuration of line 0"]
            pub fn tr(&self, n: usize) -> super::vals::Tr {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Tr(val as u8)
            }
            #[doc = "Rising trigger event configuration of line 0"]
            pub fn set_tr(&mut self, n: usize, val: super::vals::Tr) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Rtsr {
            fn default() -> Rtsr {
                Rtsr(0)
            }
        }
        #[doc = "Interrupt mask register (EXTI_IMR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Imr(pub u32);
        impl Imr {
            #[doc = "Interrupt Mask on line 0"]
            pub fn mr(&self, n: usize) -> super::vals::Mr {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Mr(val as u8)
            }
            #[doc = "Interrupt Mask on line 0"]
            pub fn set_mr(&mut self, n: usize, val: super::vals::Mr) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Imr {
            fn default() -> Imr {
                Imr(0)
            }
        }
        #[doc = "Falling Trigger selection register (EXTI_FTSR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ftsr(pub u32);
        impl Ftsr {
            #[doc = "Falling trigger event configuration of line 0"]
            pub fn tr(&self, n: usize) -> super::vals::Tr {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Tr(val as u8)
            }
            #[doc = "Falling trigger event configuration of line 0"]
            pub fn set_tr(&mut self, n: usize, val: super::vals::Tr) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Ftsr {
            fn default() -> Ftsr {
                Ftsr(0)
            }
        }
        #[doc = "Event mask register (EXTI_EMR)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Emr(pub u32);
        impl Emr {
            #[doc = "Event Mask on line 0"]
            pub fn mr(&self, n: usize) -> super::vals::Mr {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Mr(val as u8)
            }
            #[doc = "Event Mask on line 0"]
            pub fn set_mr(&mut self, n: usize, val: super::vals::Mr) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Emr {
            fn default() -> Emr {
                Emr(0)
            }
        }
        #[doc = "Software interrupt event register (EXTI_SWIER)"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Swier(pub u32);
        impl Swier {
            #[doc = "Software Interrupt on line 0"]
            pub fn swier(&self, n: usize) -> bool {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Software Interrupt on line 0"]
            pub fn set_swier(&mut self, n: usize, val: bool) {
                assert!(n < 23usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Swier {
            fn default() -> Swier {
                Swier(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Prw(pub u8);
        impl Prw {
            #[doc = "Clears pending bit"]
            pub const CLEAR: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Tr(pub u8);
        impl Tr {
            #[doc = "Falling edge trigger is disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Falling edge trigger is enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Prr(pub u8);
        impl Prr {
            #[doc = "No trigger request occurred"]
            pub const NOTPENDING: Self = Self(0);
            #[doc = "Selected trigger request occurred"]
            pub const PENDING: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mr(pub u8);
        impl Mr {
            #[doc = "Interrupt request line is masked"]
            pub const MASKED: Self = Self(0);
            #[doc = "Interrupt request line is unmasked"]
            pub const UNMASKED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Swierw(pub u8);
        impl Swierw {
            #[doc = "Generates an interrupt request"]
            pub const PEND: Self = Self(0x01);
        }
    }
}
pub mod rng {
    use crate::generic::*;
    #[doc = "Random number generator"]
    #[derive(Copy, Clone)]
    pub struct Rng(pub *mut u8);
    unsafe impl Send for Rng {}
    unsafe impl Sync for Rng {}
    impl Rng {
        #[doc = "control register"]
        pub fn cr(self) -> Reg<regs::Cr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::Sr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "data register"]
        pub fn dr(self) -> Reg<u32, R> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr(pub u32);
        impl Cr {
            #[doc = "Random number generator enable"]
            pub const fn rngen(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Random number generator enable"]
            pub fn set_rngen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Interrupt enable"]
            pub const fn ie(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Interrupt enable"]
            pub fn set_ie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Cr {
            fn default() -> Cr {
                Cr(0)
            }
        }
        #[doc = "status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sr(pub u32);
        impl Sr {
            #[doc = "Data ready"]
            pub const fn drdy(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Data ready"]
            pub fn set_drdy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Clock error current status"]
            pub const fn cecs(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Clock error current status"]
            pub fn set_cecs(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Seed error current status"]
            pub const fn secs(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Seed error current status"]
            pub fn set_secs(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Clock error interrupt status"]
            pub const fn ceis(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Clock error interrupt status"]
            pub fn set_ceis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Seed error interrupt status"]
            pub const fn seis(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Seed error interrupt status"]
            pub fn set_seis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Sr {
            fn default() -> Sr {
                Sr(0)
            }
        }
    }
}
pub mod gpio {
    use crate::generic::*;
    #[doc = "General-purpose I/Os"]
    #[derive(Copy, Clone)]
    pub struct Gpio(pub *mut u8);
    unsafe impl Send for Gpio {}
    unsafe impl Sync for Gpio {}
    impl Gpio {
        #[doc = "GPIO port mode register"]
        pub fn moder(self) -> Reg<regs::Moder, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "GPIO port output type register"]
        pub fn otyper(self) -> Reg<regs::Otyper, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "GPIO port output speed register"]
        pub fn ospeedr(self) -> Reg<regs::Ospeedr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "GPIO port pull-up/pull-down register"]
        pub fn pupdr(self) -> Reg<regs::Pupdr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "GPIO port input data register"]
        pub fn idr(self) -> Reg<regs::Idr, R> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "GPIO port output data register"]
        pub fn odr(self) -> Reg<regs::Odr, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "GPIO port bit set/reset register"]
        pub fn bsrr(self) -> Reg<regs::Bsrr, W> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "GPIO port configuration lock register"]
        pub fn lckr(self) -> Reg<regs::Lckr, RW> {
            unsafe { Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "GPIO alternate function register (low, high)"]
        pub fn afr(self, n: usize) -> Reg<regs::Afr, RW> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(32usize + n * 4usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "GPIO alternate function register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Afr(pub u32);
        impl Afr {
            #[doc = "Alternate function selection for port x bit y (y = 0..15)"]
            pub fn afr(&self, n: usize) -> super::vals::Afr {
                assert!(n < 8usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x0f;
                super::vals::Afr(val as u8)
            }
            #[doc = "Alternate function selection for port x bit y (y = 0..15)"]
            pub fn set_afr(&mut self, n: usize, val: super::vals::Afr) {
                assert!(n < 8usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x0f << offs)) | (((val.0 as u32) & 0x0f) << offs);
            }
        }
        impl Default for Afr {
            fn default() -> Afr {
                Afr(0)
            }
        }
        #[doc = "GPIO port mode register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Moder(pub u32);
        impl Moder {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn moder(&self, n: usize) -> super::vals::Moder {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Moder(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_moder(&mut self, n: usize, val: super::vals::Moder) {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
        }
        impl Default for Moder {
            fn default() -> Moder {
                Moder(0)
            }
        }
        #[doc = "GPIO port output data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Odr(pub u32);
        impl Odr {
            #[doc = "Port output data (y = 0..15)"]
            pub fn odr(&self, n: usize) -> super::vals::Odr {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Odr(val as u8)
            }
            #[doc = "Port output data (y = 0..15)"]
            pub fn set_odr(&mut self, n: usize, val: super::vals::Odr) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Odr {
            fn default() -> Odr {
                Odr(0)
            }
        }
        #[doc = "GPIO port configuration lock register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Lckr(pub u32);
        impl Lckr {
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub fn lck(&self, n: usize) -> super::vals::Lck {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Lck(val as u8)
            }
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub fn set_lck(&mut self, n: usize, val: super::vals::Lck) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub const fn lckk(&self) -> super::vals::Lckk {
                let val = (self.0 >> 16usize) & 0x01;
                super::vals::Lckk(val as u8)
            }
            #[doc = "Port x lock bit y (y= 0..15)"]
            pub fn set_lckk(&mut self, val: super::vals::Lckk) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
            }
        }
        impl Default for Lckr {
            fn default() -> Lckr {
                Lckr(0)
            }
        }
        #[doc = "GPIO port bit set/reset register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Bsrr(pub u32);
        impl Bsrr {
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn bs(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn set_bs(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn br(&self, n: usize) -> bool {
                assert!(n < 16usize);
                let offs = 16usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Port x set bit y (y= 0..15)"]
            pub fn set_br(&mut self, n: usize, val: bool) {
                assert!(n < 16usize);
                let offs = 16usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Bsrr {
            fn default() -> Bsrr {
                Bsrr(0)
            }
        }
        #[doc = "GPIO port input data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Idr(pub u32);
        impl Idr {
            #[doc = "Port input data (y = 0..15)"]
            pub fn idr(&self, n: usize) -> super::vals::Idr {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Idr(val as u8)
            }
            #[doc = "Port input data (y = 0..15)"]
            pub fn set_idr(&mut self, n: usize, val: super::vals::Idr) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Idr {
            fn default() -> Idr {
                Idr(0)
            }
        }
        #[doc = "GPIO port output type register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Otyper(pub u32);
        impl Otyper {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn ot(&self, n: usize) -> super::vals::Ot {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                let val = (self.0 >> offs) & 0x01;
                super::vals::Ot(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_ot(&mut self, n: usize, val: super::vals::Ot) {
                assert!(n < 16usize);
                let offs = 0usize + n * 1usize;
                self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
            }
        }
        impl Default for Otyper {
            fn default() -> Otyper {
                Otyper(0)
            }
        }
        #[doc = "GPIO port output speed register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ospeedr(pub u32);
        impl Ospeedr {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn ospeedr(&self, n: usize) -> super::vals::Ospeedr {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Ospeedr(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_ospeedr(&mut self, n: usize, val: super::vals::Ospeedr) {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
        }
        impl Default for Ospeedr {
            fn default() -> Ospeedr {
                Ospeedr(0)
            }
        }
        #[doc = "GPIO port pull-up/pull-down register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pupdr(pub u32);
        impl Pupdr {
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn pupdr(&self, n: usize) -> super::vals::Pupdr {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                let val = (self.0 >> offs) & 0x03;
                super::vals::Pupdr(val as u8)
            }
            #[doc = "Port x configuration bits (y = 0..15)"]
            pub fn set_pupdr(&mut self, n: usize, val: super::vals::Pupdr) {
                assert!(n < 16usize);
                let offs = 0usize + n * 2usize;
                self.0 = (self.0 & !(0x03 << offs)) | (((val.0 as u32) & 0x03) << offs);
            }
        }
        impl Default for Pupdr {
            fn default() -> Pupdr {
                Pupdr(0)
            }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pupdr(pub u8);
        impl Pupdr {
            #[doc = "No pull-up, pull-down"]
            pub const FLOATING: Self = Self(0);
            #[doc = "Pull-up"]
            pub const PULLUP: Self = Self(0x01);
            #[doc = "Pull-down"]
            pub const PULLDOWN: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Odr(pub u8);
        impl Odr {
            #[doc = "Set output to logic low"]
            pub const LOW: Self = Self(0);
            #[doc = "Set output to logic high"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Idr(pub u8);
        impl Idr {
            #[doc = "Input is logic low"]
            pub const LOW: Self = Self(0);
            #[doc = "Input is logic high"]
            pub const HIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lck(pub u8);
        impl Lck {
            #[doc = "Port configuration not locked"]
            pub const UNLOCKED: Self = Self(0);
            #[doc = "Port configuration locked"]
            pub const LOCKED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Moder(pub u8);
        impl Moder {
            #[doc = "Input mode (reset state)"]
            pub const INPUT: Self = Self(0);
            #[doc = "General purpose output mode"]
            pub const OUTPUT: Self = Self(0x01);
            #[doc = "Alternate function mode"]
            pub const ALTERNATE: Self = Self(0x02);
            #[doc = "Analog mode"]
            pub const ANALOG: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ot(pub u8);
        impl Ot {
            #[doc = "Output push-pull (reset state)"]
            pub const PUSHPULL: Self = Self(0);
            #[doc = "Output open-drain"]
            pub const OPENDRAIN: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Afr(pub u8);
        impl Afr {
            #[doc = "AF0"]
            pub const AF0: Self = Self(0);
            #[doc = "AF1"]
            pub const AF1: Self = Self(0x01);
            #[doc = "AF2"]
            pub const AF2: Self = Self(0x02);
            #[doc = "AF3"]
            pub const AF3: Self = Self(0x03);
            #[doc = "AF4"]
            pub const AF4: Self = Self(0x04);
            #[doc = "AF5"]
            pub const AF5: Self = Self(0x05);
            #[doc = "AF6"]
            pub const AF6: Self = Self(0x06);
            #[doc = "AF7"]
            pub const AF7: Self = Self(0x07);
            #[doc = "AF8"]
            pub const AF8: Self = Self(0x08);
            #[doc = "AF9"]
            pub const AF9: Self = Self(0x09);
            #[doc = "AF10"]
            pub const AF10: Self = Self(0x0a);
            #[doc = "AF11"]
            pub const AF11: Self = Self(0x0b);
            #[doc = "AF12"]
            pub const AF12: Self = Self(0x0c);
            #[doc = "AF13"]
            pub const AF13: Self = Self(0x0d);
            #[doc = "AF14"]
            pub const AF14: Self = Self(0x0e);
            #[doc = "AF15"]
            pub const AF15: Self = Self(0x0f);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lckk(pub u8);
        impl Lckk {
            #[doc = "Port configuration lock key not active"]
            pub const NOTACTIVE: Self = Self(0);
            #[doc = "Port configuration lock key active"]
            pub const ACTIVE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Bsw(pub u8);
        impl Bsw {
            #[doc = "Sets the corresponding ODRx bit"]
            pub const SET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Brw(pub u8);
        impl Brw {
            #[doc = "Resets the corresponding ODRx bit"]
            pub const RESET: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ospeedr(pub u8);
        impl Ospeedr {
            #[doc = "Low speed"]
            pub const LOWSPEED: Self = Self(0);
            #[doc = "Medium speed"]
            pub const MEDIUMSPEED: Self = Self(0x01);
            #[doc = "High speed"]
            pub const HIGHSPEED: Self = Self(0x02);
            #[doc = "Very high speed"]
            pub const VERYHIGHSPEED: Self = Self(0x03);
        }
    }
}
pub mod spi {
    use crate::generic::*;
    #[doc = "Serial peripheral interface"]
    #[derive(Copy, Clone)]
    pub struct Spi(pub *mut u8);
    unsafe impl Send for Spi {}
    unsafe impl Sync for Spi {}
    impl Spi {
        #[doc = "control register 1"]
        pub fn cr1(self) -> Reg<regs::Cr1, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "control register 2"]
        pub fn cr2(self) -> Reg<regs::Cr2, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "status register"]
        pub fn sr(self) -> Reg<regs::Sr, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "data register"]
        pub fn dr(self) -> Reg<regs::Dr, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "CRC polynomial register"]
        pub fn crcpr(self) -> Reg<regs::Crcpr, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "RX CRC register"]
        pub fn rxcrcr(self) -> Reg<regs::Rxcrcr, R> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "TX CRC register"]
        pub fn txcrcr(self) -> Reg<regs::Txcrcr, R> {
            unsafe { Reg::from_ptr(self.0.add(24usize)) }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lsbfirst(pub u8);
        impl Lsbfirst {
            #[doc = "Data is transmitted/received with the MSB first"]
            pub const MSBFIRST: Self = Self(0);
            #[doc = "Data is transmitted/received with the LSB first"]
            pub const LSBFIRST: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Iscfg(pub u8);
        impl Iscfg {
            #[doc = "Slave - transmit"]
            pub const SLAVETX: Self = Self(0);
            #[doc = "Slave - receive"]
            pub const SLAVERX: Self = Self(0x01);
            #[doc = "Master - transmit"]
            pub const MASTERTX: Self = Self(0x02);
            #[doc = "Master - receive"]
            pub const MASTERRX: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpol(pub u8);
        impl Cpol {
            #[doc = "CK to 0 when idle"]
            pub const IDLELOW: Self = Self(0);
            #[doc = "CK to 1 when idle"]
            pub const IDLEHIGH: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Bidioe(pub u8);
        impl Bidioe {
            #[doc = "Output disabled (receive-only mode)"]
            pub const OUTPUTDISABLED: Self = Self(0);
            #[doc = "Output enabled (transmit-only mode)"]
            pub const OUTPUTENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Crcnext(pub u8);
        impl Crcnext {
            #[doc = "Next transmit value is from Tx buffer"]
            pub const TXBUFFER: Self = Self(0);
            #[doc = "Next transmit value is from Tx CRC register"]
            pub const CRC: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Rxonly(pub u8);
        impl Rxonly {
            #[doc = "Full duplex (Transmit and receive)"]
            pub const FULLDUPLEX: Self = Self(0);
            #[doc = "Output disabled (Receive-only mode)"]
            pub const OUTPUTDISABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cpha(pub u8);
        impl Cpha {
            #[doc = "The first clock transition is the first data capture edge"]
            pub const FIRSTEDGE: Self = Self(0);
            #[doc = "The second clock transition is the first data capture edge"]
            pub const SECONDEDGE: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Br(pub u8);
        impl Br {
            #[doc = "f_PCLK / 2"]
            pub const DIV2: Self = Self(0);
            #[doc = "f_PCLK / 4"]
            pub const DIV4: Self = Self(0x01);
            #[doc = "f_PCLK / 8"]
            pub const DIV8: Self = Self(0x02);
            #[doc = "f_PCLK / 16"]
            pub const DIV16: Self = Self(0x03);
            #[doc = "f_PCLK / 32"]
            pub const DIV32: Self = Self(0x04);
            #[doc = "f_PCLK / 64"]
            pub const DIV64: Self = Self(0x05);
            #[doc = "f_PCLK / 128"]
            pub const DIV128: Self = Self(0x06);
            #[doc = "f_PCLK / 256"]
            pub const DIV256: Self = Self(0x07);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Bidimode(pub u8);
        impl Bidimode {
            #[doc = "2-line unidirectional data mode selected"]
            pub const UNIDIRECTIONAL: Self = Self(0);
            #[doc = "1-line bidirectional data mode selected"]
            pub const BIDIRECTIONAL: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frer(pub u8);
        impl Frer {
            #[doc = "No frame format error"]
            pub const NOERROR: Self = Self(0);
            #[doc = "A frame format error occurred"]
            pub const ERROR: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mstr(pub u8);
        impl Mstr {
            #[doc = "Slave configuration"]
            pub const SLAVE: Self = Self(0);
            #[doc = "Master configuration"]
            pub const MASTER: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dff(pub u8);
        impl Dff {
            #[doc = "8-bit data frame format is selected for transmission/reception"]
            pub const EIGHTBIT: Self = Self(0);
            #[doc = "16-bit data frame format is selected for transmission/reception"]
            pub const SIXTEENBIT: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Frf(pub u8);
        impl Frf {
            #[doc = "SPI Motorola mode"]
            pub const MOTOROLA: Self = Self(0);
            #[doc = "SPI TI mode"]
            pub const TI: Self = Self(0x01);
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dr(pub u32);
        impl Dr {
            #[doc = "Data register"]
            pub const fn dr(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Data register"]
            pub fn set_dr(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Dr {
            fn default() -> Dr {
                Dr(0)
            }
        }
        #[doc = "control register 2"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr2(pub u32);
        impl Cr2 {
            #[doc = "Rx buffer DMA enable"]
            pub const fn rxdmaen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Rx buffer DMA enable"]
            pub fn set_rxdmaen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Tx buffer DMA enable"]
            pub const fn txdmaen(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Tx buffer DMA enable"]
            pub fn set_txdmaen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "SS output enable"]
            pub const fn ssoe(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "SS output enable"]
            pub fn set_ssoe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Frame format"]
            pub const fn frf(&self) -> super::vals::Frf {
                let val = (self.0 >> 4usize) & 0x01;
                super::vals::Frf(val as u8)
            }
            #[doc = "Frame format"]
            pub fn set_frf(&mut self, val: super::vals::Frf) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
            }
            #[doc = "Error interrupt enable"]
            pub const fn errie(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Error interrupt enable"]
            pub fn set_errie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "RX buffer not empty interrupt enable"]
            pub const fn rxneie(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "RX buffer not empty interrupt enable"]
            pub fn set_rxneie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Tx buffer empty interrupt enable"]
            pub const fn txeie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Tx buffer empty interrupt enable"]
            pub fn set_txeie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Cr2 {
            fn default() -> Cr2 {
                Cr2(0)
            }
        }
        #[doc = "TX CRC register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Txcrcr(pub u32);
        impl Txcrcr {
            #[doc = "Tx CRC register"]
            pub const fn tx_crc(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Tx CRC register"]
            pub fn set_tx_crc(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Txcrcr {
            fn default() -> Txcrcr {
                Txcrcr(0)
            }
        }
        #[doc = "control register 1"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr1(pub u32);
        impl Cr1 {
            #[doc = "Clock phase"]
            pub const fn cpha(&self) -> super::vals::Cpha {
                let val = (self.0 >> 0usize) & 0x01;
                super::vals::Cpha(val as u8)
            }
            #[doc = "Clock phase"]
            pub fn set_cpha(&mut self, val: super::vals::Cpha) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
            }
            #[doc = "Clock polarity"]
            pub const fn cpol(&self) -> super::vals::Cpol {
                let val = (self.0 >> 1usize) & 0x01;
                super::vals::Cpol(val as u8)
            }
            #[doc = "Clock polarity"]
            pub fn set_cpol(&mut self, val: super::vals::Cpol) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
            }
            #[doc = "Master selection"]
            pub const fn mstr(&self) -> super::vals::Mstr {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Mstr(val as u8)
            }
            #[doc = "Master selection"]
            pub fn set_mstr(&mut self, val: super::vals::Mstr) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "Baud rate control"]
            pub const fn br(&self) -> super::vals::Br {
                let val = (self.0 >> 3usize) & 0x07;
                super::vals::Br(val as u8)
            }
            #[doc = "Baud rate control"]
            pub fn set_br(&mut self, val: super::vals::Br) {
                self.0 = (self.0 & !(0x07 << 3usize)) | (((val.0 as u32) & 0x07) << 3usize);
            }
            #[doc = "SPI enable"]
            pub const fn spe(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "SPI enable"]
            pub fn set_spe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Frame format"]
            pub const fn lsbfirst(&self) -> super::vals::Lsbfirst {
                let val = (self.0 >> 7usize) & 0x01;
                super::vals::Lsbfirst(val as u8)
            }
            #[doc = "Frame format"]
            pub fn set_lsbfirst(&mut self, val: super::vals::Lsbfirst) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
            }
            #[doc = "Internal slave select"]
            pub const fn ssi(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Internal slave select"]
            pub fn set_ssi(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "Software slave management"]
            pub const fn ssm(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "Software slave management"]
            pub fn set_ssm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "Receive only"]
            pub const fn rxonly(&self) -> super::vals::Rxonly {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Rxonly(val as u8)
            }
            #[doc = "Receive only"]
            pub fn set_rxonly(&mut self, val: super::vals::Rxonly) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "Data frame format"]
            pub const fn dff(&self) -> super::vals::Dff {
                let val = (self.0 >> 11usize) & 0x01;
                super::vals::Dff(val as u8)
            }
            #[doc = "Data frame format"]
            pub fn set_dff(&mut self, val: super::vals::Dff) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val.0 as u32) & 0x01) << 11usize);
            }
            #[doc = "CRC transfer next"]
            pub const fn crcnext(&self) -> super::vals::Crcnext {
                let val = (self.0 >> 12usize) & 0x01;
                super::vals::Crcnext(val as u8)
            }
            #[doc = "CRC transfer next"]
            pub fn set_crcnext(&mut self, val: super::vals::Crcnext) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
            }
            #[doc = "Hardware CRC calculation enable"]
            pub const fn crcen(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "Hardware CRC calculation enable"]
            pub fn set_crcen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
            #[doc = "Output enable in bidirectional mode"]
            pub const fn bidioe(&self) -> super::vals::Bidioe {
                let val = (self.0 >> 14usize) & 0x01;
                super::vals::Bidioe(val as u8)
            }
            #[doc = "Output enable in bidirectional mode"]
            pub fn set_bidioe(&mut self, val: super::vals::Bidioe) {
                self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
            }
            #[doc = "Bidirectional data mode enable"]
            pub const fn bidimode(&self) -> super::vals::Bidimode {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Bidimode(val as u8)
            }
            #[doc = "Bidirectional data mode enable"]
            pub fn set_bidimode(&mut self, val: super::vals::Bidimode) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
        }
        impl Default for Cr1 {
            fn default() -> Cr1 {
                Cr1(0)
            }
        }
        #[doc = "RX CRC register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxcrcr(pub u32);
        impl Rxcrcr {
            #[doc = "Rx CRC register"]
            pub const fn rx_crc(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Rx CRC register"]
            pub fn set_rx_crc(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Rxcrcr {
            fn default() -> Rxcrcr {
                Rxcrcr(0)
            }
        }
        #[doc = "status register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sr(pub u32);
        impl Sr {
            #[doc = "Receive buffer not empty"]
            pub const fn rxne(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Receive buffer not empty"]
            pub fn set_rxne(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Transmit buffer empty"]
            pub const fn txe(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Transmit buffer empty"]
            pub fn set_txe(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "CRC error flag"]
            pub const fn crcerr(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "CRC error flag"]
            pub fn set_crcerr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Mode fault"]
            pub const fn modf(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "Mode fault"]
            pub fn set_modf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "Overrun flag"]
            pub const fn ovr(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Overrun flag"]
            pub fn set_ovr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Busy flag"]
            pub const fn bsy(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "Busy flag"]
            pub fn set_bsy(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "TI frame format error"]
            pub const fn fre(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "TI frame format error"]
            pub fn set_fre(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Sr {
            fn default() -> Sr {
                Sr(0)
            }
        }
        #[doc = "CRC polynomial register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Crcpr(pub u32);
        impl Crcpr {
            #[doc = "CRC polynomial register"]
            pub const fn crcpoly(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "CRC polynomial register"]
            pub fn set_crcpoly(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Crcpr {
            fn default() -> Crcpr {
                Crcpr(0)
            }
        }
    }
}
pub mod dma {
    use crate::generic::*;
    #[doc = "DMA controller"]
    #[derive(Copy, Clone)]
    pub struct Dma(pub *mut u8);
    unsafe impl Send for Dma {}
    unsafe impl Sync for Dma {}
    impl Dma {
        #[doc = "low interrupt status register"]
        pub fn isr(self, n: usize) -> Reg<regs::Ixr, R> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(0usize + n * 4usize)) }
        }
        #[doc = "low interrupt flag clear register"]
        pub fn ifcr(self, n: usize) -> Reg<regs::Ixr, W> {
            assert!(n < 2usize);
            unsafe { Reg::from_ptr(self.0.add(8usize + n * 4usize)) }
        }
        #[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
        pub fn st(self, n: usize) -> St {
            assert!(n < 8usize);
            unsafe { St(self.0.add(16usize + n * 24usize)) }
        }
    }
    #[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
    #[derive(Copy, Clone)]
    pub struct St(pub *mut u8);
    unsafe impl Send for St {}
    unsafe impl Sync for St {}
    impl St {
        #[doc = "stream x configuration register"]
        pub fn cr(self) -> Reg<regs::Cr, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "stream x number of data register"]
        pub fn ndtr(self) -> Reg<regs::Ndtr, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "stream x peripheral address register"]
        pub fn par(self) -> Reg<u32, RW> {
            unsafe { Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "stream x memory 0 address register"]
        pub fn m0ar(self) -> Reg<u32, RW> {
            unsafe { Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "stream x memory 1 address register"]
        pub fn m1ar(self) -> Reg<u32, RW> {
            unsafe { Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "stream x FIFO control register"]
        pub fn fcr(self) -> Reg<regs::Fcr, RW> {
            unsafe { Reg::from_ptr(self.0.add(20usize)) }
        }
    }
    pub mod vals {
        use crate::generic::*;
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Burst(pub u8);
        impl Burst {
            #[doc = "Single transfer"]
            pub const SINGLE: Self = Self(0);
            #[doc = "Incremental burst of 4 beats"]
            pub const INCR4: Self = Self(0x01);
            #[doc = "Incremental burst of 8 beats"]
            pub const INCR8: Self = Self(0x02);
            #[doc = "Incremental burst of 16 beats"]
            pub const INCR16: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dir(pub u8);
        impl Dir {
            #[doc = "Peripheral-to-memory"]
            pub const PERIPHERALTOMEMORY: Self = Self(0);
            #[doc = "Memory-to-peripheral"]
            pub const MEMORYTOPERIPHERAL: Self = Self(0x01);
            #[doc = "Memory-to-memory"]
            pub const MEMORYTOMEMORY: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pl(pub u8);
        impl Pl {
            #[doc = "Low"]
            pub const LOW: Self = Self(0);
            #[doc = "Medium"]
            pub const MEDIUM: Self = Self(0x01);
            #[doc = "High"]
            pub const HIGH: Self = Self(0x02);
            #[doc = "Very high"]
            pub const VERYHIGH: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Size(pub u8);
        impl Size {
            #[doc = "Byte (8-bit)"]
            pub const BITS8: Self = Self(0);
            #[doc = "Half-word (16-bit)"]
            pub const BITS16: Self = Self(0x01);
            #[doc = "Word (32-bit)"]
            pub const BITS32: Self = Self(0x02);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dbm(pub u8);
        impl Dbm {
            #[doc = "No buffer switching at the end of transfer"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Memory target switched at the end of the DMA transfer"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Fs(pub u8);
        impl Fs {
            #[doc = "0 < fifo_level < 1/4"]
            pub const QUARTER1: Self = Self(0);
            #[doc = "1/4 <= fifo_level < 1/2"]
            pub const QUARTER2: Self = Self(0x01);
            #[doc = "1/2 <= fifo_level < 3/4"]
            pub const QUARTER3: Self = Self(0x02);
            #[doc = "3/4 <= fifo_level < full"]
            pub const QUARTER4: Self = Self(0x03);
            #[doc = "FIFO is empty"]
            pub const EMPTY: Self = Self(0x04);
            #[doc = "FIFO is full"]
            pub const FULL: Self = Self(0x05);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pfctrl(pub u8);
        impl Pfctrl {
            #[doc = "The DMA is the flow controller"]
            pub const DMA: Self = Self(0);
            #[doc = "The peripheral is the flow controller"]
            pub const PERIPHERAL: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Inc(pub u8);
        impl Inc {
            #[doc = "Address pointer is fixed"]
            pub const FIXED: Self = Self(0);
            #[doc = "Address pointer is incremented after each data transfer"]
            pub const INCREMENTED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Circ(pub u8);
        impl Circ {
            #[doc = "Circular mode disabled"]
            pub const DISABLED: Self = Self(0);
            #[doc = "Circular mode enabled"]
            pub const ENABLED: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pincos(pub u8);
        impl Pincos {
            #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
            pub const PSIZE: Self = Self(0);
            #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
            pub const FIXED4: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ct(pub u8);
        impl Ct {
            #[doc = "The current target memory is Memory 0"]
            pub const MEMORY0: Self = Self(0);
            #[doc = "The current target memory is Memory 1"]
            pub const MEMORY1: Self = Self(0x01);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Fth(pub u8);
        impl Fth {
            #[doc = "1/4 full FIFO"]
            pub const QUARTER: Self = Self(0);
            #[doc = "1/2 full FIFO"]
            pub const HALF: Self = Self(0x01);
            #[doc = "3/4 full FIFO"]
            pub const THREEQUARTERS: Self = Self(0x02);
            #[doc = "Full FIFO"]
            pub const FULL: Self = Self(0x03);
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Dmdis(pub u8);
        impl Dmdis {
            #[doc = "Direct mode is enabled"]
            pub const ENABLED: Self = Self(0);
            #[doc = "Direct mode is disabled"]
            pub const DISABLED: Self = Self(0x01);
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "interrupt register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ixr(pub u32);
        impl Ixr {
            #[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
            pub fn feif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 0usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Stream x FIFO error interrupt flag (x=3..0)"]
            pub fn set_feif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 0usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
            pub fn dmeif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 2usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Stream x direct mode error interrupt flag (x=3..0)"]
            pub fn set_dmeif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 2usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Stream x transfer error interrupt flag (x=3..0)"]
            pub fn teif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 3usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Stream x transfer error interrupt flag (x=3..0)"]
            pub fn set_teif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 3usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Stream x half transfer interrupt flag (x=3..0)"]
            pub fn htif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 4usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Stream x half transfer interrupt flag (x=3..0)"]
            pub fn set_htif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 4usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
            #[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
            pub fn tcif(&self, n: usize) -> bool {
                assert!(n < 4usize);
                let offs = 5usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                let val = (self.0 >> offs) & 0x01;
                val != 0
            }
            #[doc = "Stream x transfer complete interrupt flag (x = 3..0)"]
            pub fn set_tcif(&mut self, n: usize, val: bool) {
                assert!(n < 4usize);
                let offs = 5usize + ([0usize, 6usize, 16usize, 22usize][n] as usize);
                self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
            }
        }
        impl Default for Ixr {
            fn default() -> Ixr {
                Ixr(0)
            }
        }
        #[doc = "stream x configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cr(pub u32);
        impl Cr {
            #[doc = "Stream enable / flag stream ready when read low"]
            pub const fn en(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Stream enable / flag stream ready when read low"]
            pub fn set_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Direct mode error interrupt enable"]
            pub const fn dmeie(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Direct mode error interrupt enable"]
            pub fn set_dmeie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "Transfer error interrupt enable"]
            pub const fn teie(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Transfer error interrupt enable"]
            pub fn set_teie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Half transfer interrupt enable"]
            pub const fn htie(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Half transfer interrupt enable"]
            pub fn set_htie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Transfer complete interrupt enable"]
            pub const fn tcie(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Transfer complete interrupt enable"]
            pub fn set_tcie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "Peripheral flow controller"]
            pub const fn pfctrl(&self) -> super::vals::Pfctrl {
                let val = (self.0 >> 5usize) & 0x01;
                super::vals::Pfctrl(val as u8)
            }
            #[doc = "Peripheral flow controller"]
            pub fn set_pfctrl(&mut self, val: super::vals::Pfctrl) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
            }
            #[doc = "Data transfer direction"]
            pub const fn dir(&self) -> super::vals::Dir {
                let val = (self.0 >> 6usize) & 0x03;
                super::vals::Dir(val as u8)
            }
            #[doc = "Data transfer direction"]
            pub fn set_dir(&mut self, val: super::vals::Dir) {
                self.0 = (self.0 & !(0x03 << 6usize)) | (((val.0 as u32) & 0x03) << 6usize);
            }
            #[doc = "Circular mode"]
            pub const fn circ(&self) -> super::vals::Circ {
                let val = (self.0 >> 8usize) & 0x01;
                super::vals::Circ(val as u8)
            }
            #[doc = "Circular mode"]
            pub fn set_circ(&mut self, val: super::vals::Circ) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
            }
            #[doc = "Peripheral increment mode"]
            pub const fn pinc(&self) -> super::vals::Inc {
                let val = (self.0 >> 9usize) & 0x01;
                super::vals::Inc(val as u8)
            }
            #[doc = "Peripheral increment mode"]
            pub fn set_pinc(&mut self, val: super::vals::Inc) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val.0 as u32) & 0x01) << 9usize);
            }
            #[doc = "Memory increment mode"]
            pub const fn minc(&self) -> super::vals::Inc {
                let val = (self.0 >> 10usize) & 0x01;
                super::vals::Inc(val as u8)
            }
            #[doc = "Memory increment mode"]
            pub fn set_minc(&mut self, val: super::vals::Inc) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val.0 as u32) & 0x01) << 10usize);
            }
            #[doc = "Peripheral data size"]
            pub const fn psize(&self) -> super::vals::Size {
                let val = (self.0 >> 11usize) & 0x03;
                super::vals::Size(val as u8)
            }
            #[doc = "Peripheral data size"]
            pub fn set_psize(&mut self, val: super::vals::Size) {
                self.0 = (self.0 & !(0x03 << 11usize)) | (((val.0 as u32) & 0x03) << 11usize);
            }
            #[doc = "Memory data size"]
            pub const fn msize(&self) -> super::vals::Size {
                let val = (self.0 >> 13usize) & 0x03;
                super::vals::Size(val as u8)
            }
            #[doc = "Memory data size"]
            pub fn set_msize(&mut self, val: super::vals::Size) {
                self.0 = (self.0 & !(0x03 << 13usize)) | (((val.0 as u32) & 0x03) << 13usize);
            }
            #[doc = "Peripheral increment offset size"]
            pub const fn pincos(&self) -> super::vals::Pincos {
                let val = (self.0 >> 15usize) & 0x01;
                super::vals::Pincos(val as u8)
            }
            #[doc = "Peripheral increment offset size"]
            pub fn set_pincos(&mut self, val: super::vals::Pincos) {
                self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
            }
            #[doc = "Priority level"]
            pub const fn pl(&self) -> super::vals::Pl {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Pl(val as u8)
            }
            #[doc = "Priority level"]
            pub fn set_pl(&mut self, val: super::vals::Pl) {
                self.0 = (self.0 & !(0x03 << 16usize)) | (((val.0 as u32) & 0x03) << 16usize);
            }
            #[doc = "Double buffer mode"]
            pub const fn dbm(&self) -> super::vals::Dbm {
                let val = (self.0 >> 18usize) & 0x01;
                super::vals::Dbm(val as u8)
            }
            #[doc = "Double buffer mode"]
            pub fn set_dbm(&mut self, val: super::vals::Dbm) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val.0 as u32) & 0x01) << 18usize);
            }
            #[doc = "Current target (only in double buffer mode)"]
            pub const fn ct(&self) -> super::vals::Ct {
                let val = (self.0 >> 19usize) & 0x01;
                super::vals::Ct(val as u8)
            }
            #[doc = "Current target (only in double buffer mode)"]
            pub fn set_ct(&mut self, val: super::vals::Ct) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val.0 as u32) & 0x01) << 19usize);
            }
            #[doc = "Peripheral burst transfer configuration"]
            pub const fn pburst(&self) -> super::vals::Burst {
                let val = (self.0 >> 21usize) & 0x03;
                super::vals::Burst(val as u8)
            }
            #[doc = "Peripheral burst transfer configuration"]
            pub fn set_pburst(&mut self, val: super::vals::Burst) {
                self.0 = (self.0 & !(0x03 << 21usize)) | (((val.0 as u32) & 0x03) << 21usize);
            }
            #[doc = "Memory burst transfer configuration"]
            pub const fn mburst(&self) -> super::vals::Burst {
                let val = (self.0 >> 23usize) & 0x03;
                super::vals::Burst(val as u8)
            }
            #[doc = "Memory burst transfer configuration"]
            pub fn set_mburst(&mut self, val: super::vals::Burst) {
                self.0 = (self.0 & !(0x03 << 23usize)) | (((val.0 as u32) & 0x03) << 23usize);
            }
            #[doc = "Channel selection"]
            pub const fn chsel(&self) -> u8 {
                let val = (self.0 >> 25usize) & 0x0f;
                val as u8
            }
            #[doc = "Channel selection"]
            pub fn set_chsel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
            }
        }
        impl Default for Cr {
            fn default() -> Cr {
                Cr(0)
            }
        }
        #[doc = "stream x FIFO control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Fcr(pub u32);
        impl Fcr {
            #[doc = "FIFO threshold selection"]
            pub const fn fth(&self) -> super::vals::Fth {
                let val = (self.0 >> 0usize) & 0x03;
                super::vals::Fth(val as u8)
            }
            #[doc = "FIFO threshold selection"]
            pub fn set_fth(&mut self, val: super::vals::Fth) {
                self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
            }
            #[doc = "Direct mode disable"]
            pub const fn dmdis(&self) -> super::vals::Dmdis {
                let val = (self.0 >> 2usize) & 0x01;
                super::vals::Dmdis(val as u8)
            }
            #[doc = "Direct mode disable"]
            pub fn set_dmdis(&mut self, val: super::vals::Dmdis) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
            }
            #[doc = "FIFO status"]
            pub const fn fs(&self) -> super::vals::Fs {
                let val = (self.0 >> 3usize) & 0x07;
                super::vals::Fs(val as u8)
            }
            #[doc = "FIFO status"]
            pub fn set_fs(&mut self, val: super::vals::Fs) {
                self.0 = (self.0 & !(0x07 << 3usize)) | (((val.0 as u32) & 0x07) << 3usize);
            }
            #[doc = "FIFO error interrupt enable"]
            pub const fn feie(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "FIFO error interrupt enable"]
            pub fn set_feie(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
        }
        impl Default for Fcr {
            fn default() -> Fcr {
                Fcr(0)
            }
        }
        #[doc = "stream x number of data register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ndtr(pub u32);
        impl Ndtr {
            #[doc = "Number of data items to transfer"]
            pub const fn ndt(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Number of data items to transfer"]
            pub fn set_ndt(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
        }
        impl Default for Ndtr {
            fn default() -> Ndtr {
                Ndtr(0)
            }
        }
    }
}
pub mod generic {
    use core::marker::PhantomData;
    #[derive(Copy, Clone)]
    pub struct RW;
    #[derive(Copy, Clone)]
    pub struct R;
    #[derive(Copy, Clone)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        pub fn from_ptr(ptr: *mut u8) -> Self {
            Self {
                ptr,
                phantom: PhantomData,
            }
        }
        pub fn ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        pub unsafe fn read(&self) -> T {
            (self.ptr as *mut T).read_volatile()
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        pub unsafe fn write_value(&self, val: T) {
            (self.ptr as *mut T).write_volatile(val)
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        pub unsafe fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = Default::default();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        pub unsafe fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = self.read();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
}
pub mod syscfg {
    use crate::generic::*;
    #[doc = "System configuration controller"]
    #[derive(Copy, Clone)]
    pub struct Syscfg(pub *mut u8);
    unsafe impl Send for Syscfg {}
    unsafe impl Sync for Syscfg {}
    impl Syscfg {
        #[doc = "memory remap register"]
        pub fn memrm(self) -> Reg<regs::Memrm, RW> {
            unsafe { Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "peripheral mode configuration register"]
        pub fn pmc(self) -> Reg<regs::Pmc, RW> {
            unsafe { Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "external interrupt configuration register"]
        pub fn exticr(self, n: usize) -> Reg<regs::Exticr, RW> {
            assert!(n < 4usize);
            unsafe { Reg::from_ptr(self.0.add(8usize + n * 4usize)) }
        }
        #[doc = "Compensation cell control register"]
        pub fn cmpcr(self) -> Reg<regs::Cmpcr, R> {
            unsafe { Reg::from_ptr(self.0.add(32usize)) }
        }
    }
    pub mod regs {
        use crate::generic::*;
        #[doc = "memory remap register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Memrm(pub u32);
        impl Memrm {
            #[doc = "Memory mapping selection"]
            pub const fn mem_mode(&self) -> u8 {
                let val = (self.0 >> 0usize) & 0x07;
                val as u8
            }
            #[doc = "Memory mapping selection"]
            pub fn set_mem_mode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
            }
            #[doc = "Flash bank mode selection"]
            pub const fn fb_mode(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "Flash bank mode selection"]
            pub fn set_fb_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "FMC memory mapping swap"]
            pub const fn swp_fmc(&self) -> u8 {
                let val = (self.0 >> 10usize) & 0x03;
                val as u8
            }
            #[doc = "FMC memory mapping swap"]
            pub fn set_swp_fmc(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
            }
        }
        impl Default for Memrm {
            fn default() -> Memrm {
                Memrm(0)
            }
        }
        #[doc = "external interrupt configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Exticr(pub u32);
        impl Exticr {
            #[doc = "EXTI x configuration"]
            pub fn exti(&self, n: usize) -> u8 {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                let val = (self.0 >> offs) & 0x0f;
                val as u8
            }
            #[doc = "EXTI x configuration"]
            pub fn set_exti(&mut self, n: usize, val: u8) {
                assert!(n < 4usize);
                let offs = 0usize + n * 4usize;
                self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
            }
        }
        impl Default for Exticr {
            fn default() -> Exticr {
                Exticr(0)
            }
        }
        #[doc = "Compensation cell control register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Cmpcr(pub u32);
        impl Cmpcr {
            #[doc = "Compensation cell power-down"]
            pub const fn cmp_pd(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Compensation cell power-down"]
            pub fn set_cmp_pd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "READY"]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "READY"]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
        }
        impl Default for Cmpcr {
            fn default() -> Cmpcr {
                Cmpcr(0)
            }
        }
        #[doc = "peripheral mode configuration register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Pmc(pub u32);
        impl Pmc {
            #[doc = "ADC1DC2"]
            pub const fn adc1dc2(&self) -> bool {
                let val = (self.0 >> 16usize) & 0x01;
                val != 0
            }
            #[doc = "ADC1DC2"]
            pub fn set_adc1dc2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
            }
            #[doc = "ADC2DC2"]
            pub const fn adc2dc2(&self) -> bool {
                let val = (self.0 >> 17usize) & 0x01;
                val != 0
            }
            #[doc = "ADC2DC2"]
            pub fn set_adc2dc2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
            }
            #[doc = "ADC3DC2"]
            pub const fn adc3dc2(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "ADC3DC2"]
            pub fn set_adc3dc2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Ethernet PHY interface selection"]
            pub const fn mii_rmii_sel(&self) -> bool {
                let val = (self.0 >> 23usize) & 0x01;
                val != 0
            }
            #[doc = "Ethernet PHY interface selection"]
            pub fn set_mii_rmii_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
            }
        }
        impl Default for Pmc {
            fn default() -> Pmc {
                Pmc(0)
            }
        }
    }
}
