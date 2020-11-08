#![doc = "Peripheral access API for GD32E230 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDGT();
    fn LVD();
    fn RTC();
    fn FMC();
    fn RCU();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn DMA_CHANNEL0();
    fn DMA_CHANNEL1_2();
    fn DMA_CHANNEL3_4();
    fn ADC_CMP();
    fn TIMER0_BRK_UP_TRG_COM();
    fn TIMER0_CC();
    fn TIMER2();
    fn TIMER5();
    fn TIMER13();
    fn TIMER14();
    fn TIMER15();
    fn TIMER16();
    fn I2C0_EV();
    fn I2C1_EV();
    fn SPI0();
    fn SPI1();
    fn USART0();
    fn USART1();
    fn I2C0_ER();
    fn I2C1_ER();
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
pub static __INTERRUPTS: [Vector; 35] = [
    Vector { _handler: WWDGT },
    Vector { _handler: LVD },
    Vector { _handler: RTC },
    Vector { _handler: FMC },
    Vector { _handler: RCU },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_15 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA_CHANNEL0,
    },
    Vector {
        _handler: DMA_CHANNEL1_2,
    },
    Vector {
        _handler: DMA_CHANNEL3_4,
    },
    Vector { _handler: ADC_CMP },
    Vector {
        _handler: TIMER0_BRK_UP_TRG_COM,
    },
    Vector {
        _handler: TIMER0_CC,
    },
    Vector { _reserved: 0 },
    Vector { _handler: TIMER2 },
    Vector { _handler: TIMER5 },
    Vector { _reserved: 0 },
    Vector { _handler: TIMER13 },
    Vector { _handler: TIMER14 },
    Vector { _handler: TIMER15 },
    Vector { _handler: TIMER16 },
    Vector { _handler: I2C0_EV },
    Vector { _handler: I2C1_EV },
    Vector { _handler: SPI0 },
    Vector { _handler: SPI1 },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C0_ER },
    Vector { _reserved: 0 },
    Vector { _handler: I2C1_ER },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - WWDGT"]
    WWDGT = 0,
    #[doc = "1 - LVD"]
    LVD = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - FMC"]
    FMC = 3,
    #[doc = "4 - RCU"]
    RCU = 4,
    #[doc = "5 - EXTI0_1"]
    EXTI0_1 = 5,
    #[doc = "6 - EXTI2_3"]
    EXTI2_3 = 6,
    #[doc = "7 - EXTI4_15"]
    EXTI4_15 = 7,
    #[doc = "9 - DMA_Channel0"]
    DMA_CHANNEL0 = 9,
    #[doc = "10 - DMA_Channel1_2"]
    DMA_CHANNEL1_2 = 10,
    #[doc = "11 - DMA_Channel3_4"]
    DMA_CHANNEL3_4 = 11,
    #[doc = "12 - ADC_CMP"]
    ADC_CMP = 12,
    #[doc = "13 - TIMER0_BRK_UP_TRG_COM"]
    TIMER0_BRK_UP_TRG_COM = 13,
    #[doc = "14 - TIMER0_CC"]
    TIMER0_CC = 14,
    #[doc = "16 - TIMER2"]
    TIMER2 = 16,
    #[doc = "17 - TIMER5"]
    TIMER5 = 17,
    #[doc = "19 - TIMER13"]
    TIMER13 = 19,
    #[doc = "20 - TIMER14"]
    TIMER14 = 20,
    #[doc = "21 - TIMER15"]
    TIMER15 = 21,
    #[doc = "22 - TIMER16"]
    TIMER16 = 22,
    #[doc = "23 - I2C0_EV"]
    I2C0_EV = 23,
    #[doc = "24 - I2C1_EV"]
    I2C1_EV = 24,
    #[doc = "25 - SPI0"]
    SPI0 = 25,
    #[doc = "26 - SPI1"]
    SPI1 = 26,
    #[doc = "27 - USART0"]
    USART0 = 27,
    #[doc = "28 - USART1"]
    USART1 = 28,
    #[doc = "32 - I2C0_ER"]
    I2C0_ER = 32,
    #[doc = "34 - I2C1_ER"]
    I2C1_ER = 34,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Analog to digital converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4001_2400 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog to digital converter"]
pub mod adc;
#[doc = "Comparator"]
pub struct CMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP {}
impl CMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp::RegisterBlock {
        0x4001_001c as *const _
    }
}
impl Deref for CMP {
    type Target = cmp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP::ptr() }
    }
}
#[doc = "Comparator"]
pub mod cmp;
#[doc = "cyclic redundancy check calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4002_3000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "cyclic redundancy check calculation unit"]
pub mod crc;
#[doc = "Debug support"]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dbgmcu::RegisterBlock {
        0x4001_5800 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DBGMCU::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbgmcu;
#[doc = "DMA controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x4001_0400 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "FMC"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "FMC"]
pub mod fmc;
#[doc = "free watchdog timer"]
pub struct FWDGT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FWDGT {}
impl FWDGT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fwdgt::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for FWDGT {
    type Target = fwdgt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FWDGT::ptr() }
    }
}
#[doc = "free watchdog timer"]
pub mod fwdgt;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4800_0000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x4800_0400 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_0800 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioc;
#[doc = "General-purpose I/Os"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiof::RegisterBlock {
        0x4800_1400 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpiof::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiof;
#[doc = "Inter integrated circuit"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4000_5400 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter integrated circuit"]
pub mod i2c0;
#[doc = "Inter integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4000_5800 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Power management unit"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmu::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMU::ptr() }
    }
}
#[doc = "Power management unit"]
pub mod pmu;
#[doc = "Reset and clock unit"]
pub struct RCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCU {}
impl RCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcu::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for RCU {
    type Target = rcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCU::ptr() }
    }
}
#[doc = "Reset and clock unit"]
pub mod rcu;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "Serial peripheral interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi0;
#[doc = "Serial Peripheral Interface 1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_3800 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 1"]
pub mod spi1;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Advanced-timers"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4001_2c00 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod timer0;
#[doc = "General-purpose-timers"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer2::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod timer2;
#[doc = "Basic-timers"]
pub struct TIMER5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER5 {}
impl TIMER5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer5::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for TIMER5 {
    type Target = timer5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER5::ptr() }
    }
}
#[doc = "Basic-timers"]
pub mod timer5;
#[doc = "General-purpose-timers"]
pub struct TIMER13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER13 {}
impl TIMER13 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer13::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for TIMER13 {
    type Target = timer13::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER13::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod timer13;
#[doc = "General-purpose-timers"]
pub struct TIMER14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER14 {}
impl TIMER14 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer14::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for TIMER14 {
    type Target = timer14::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER14::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod timer14;
#[doc = "General-purpose-timers"]
pub struct TIMER15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER15 {}
impl TIMER15 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer15::RegisterBlock {
        0x4001_4400 as *const _
    }
}
impl Deref for TIMER15 {
    type Target = timer15::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER15::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod timer15;
#[doc = "General-purpose-timers"]
pub struct TIMER16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER16 {}
impl TIMER16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer15::RegisterBlock {
        0x4001_4800 as *const _
    }
}
impl Deref for TIMER16 {
    type Target = timer15::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER16::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4001_3800 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart0;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Window watchdog timer"]
pub struct WWDGT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDGT {}
impl WWDGT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdgt::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for WWDGT {
    type Target = wwdgt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDGT::ptr() }
    }
}
#[doc = "Window watchdog timer"]
pub mod wwdgt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "CMP"]
    pub CMP: CMP,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "FWDGT"]
    pub FWDGT: FWDGT,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "RCU"]
    pub RCU: RCU,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "TIMER5"]
    pub TIMER5: TIMER5,
    #[doc = "TIMER13"]
    pub TIMER13: TIMER13,
    #[doc = "TIMER14"]
    pub TIMER14: TIMER14,
    #[doc = "TIMER15"]
    pub TIMER15: TIMER15,
    #[doc = "TIMER16"]
    pub TIMER16: TIMER16,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "WWDGT"]
    pub WWDGT: WWDGT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ADC: ADC {
                _marker: PhantomData,
            },
            CMP: CMP {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            FWDGT: FWDGT {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            PMU: PMU {
                _marker: PhantomData,
            },
            RCU: RCU {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            TIMER5: TIMER5 {
                _marker: PhantomData,
            },
            TIMER13: TIMER13 {
                _marker: PhantomData,
            },
            TIMER14: TIMER14 {
                _marker: PhantomData,
            },
            TIMER15: TIMER15 {
                _marker: PhantomData,
            },
            TIMER16: TIMER16 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            WWDGT: WWDGT {
                _marker: PhantomData,
            },
        }
    }
}
