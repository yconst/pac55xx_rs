#![doc = "Peripheral access API for PAC55XX microcontrollers (generated using svd2rust v0.19.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.19.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
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
#[cfg(feature = "rt")]
extern "C" {
    fn MEMCTL();
    fn WWDT();
    fn RTC();
    fn ADC0();
    fn ADC1();
    fn ADC2();
    fn ADC3();
    fn TIMERA();
    fn TIMERB();
    fn TIMERC();
    fn TIMERD();
    fn TIMERAQEP();
    fn TIMERBQEP();
    fn TIMERCQEP();
    fn TIMERDQEP();
    fn GPIOA();
    fn GPIOB();
    fn GPIOC();
    fn GPIOD();
    fn GPIOE();
    fn GPIOF();
    fn GPIOG();
    fn I2C();
    fn USARTA();
    fn USARTB();
    fn USARTC();
    fn USARTD();
    fn CAN();
    fn GPTIMERA();
    fn GPTIMERB();
    fn SCC();
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
pub static __INTERRUPTS: [Vector; 31] = [
    Vector { _handler: MEMCTL },
    Vector { _handler: WWDT },
    Vector { _handler: RTC },
    Vector { _handler: ADC0 },
    Vector { _handler: ADC1 },
    Vector { _handler: ADC2 },
    Vector { _handler: ADC3 },
    Vector { _handler: TIMERA },
    Vector { _handler: TIMERB },
    Vector { _handler: TIMERC },
    Vector { _handler: TIMERD },
    Vector {
        _handler: TIMERAQEP,
    },
    Vector {
        _handler: TIMERBQEP,
    },
    Vector {
        _handler: TIMERCQEP,
    },
    Vector {
        _handler: TIMERDQEP,
    },
    Vector { _handler: GPIOA },
    Vector { _handler: GPIOB },
    Vector { _handler: GPIOC },
    Vector { _handler: GPIOD },
    Vector { _handler: GPIOE },
    Vector { _handler: GPIOF },
    Vector { _handler: GPIOG },
    Vector { _handler: I2C },
    Vector { _handler: USARTA },
    Vector { _handler: USARTB },
    Vector { _handler: USARTC },
    Vector { _handler: USARTD },
    Vector { _handler: CAN },
    Vector { _handler: GPTIMERA },
    Vector { _handler: GPTIMERB },
    Vector { _handler: SCC },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - MEMCTL"]
    MEMCTL = 0,
    #[doc = "1 - WWDT"]
    WWDT = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - ADC0"]
    ADC0 = 3,
    #[doc = "4 - ADC1"]
    ADC1 = 4,
    #[doc = "5 - ADC2"]
    ADC2 = 5,
    #[doc = "6 - ADC3"]
    ADC3 = 6,
    #[doc = "7 - TIMERA"]
    TIMERA = 7,
    #[doc = "8 - TIMERB"]
    TIMERB = 8,
    #[doc = "9 - TIMERC"]
    TIMERC = 9,
    #[doc = "10 - TIMERD"]
    TIMERD = 10,
    #[doc = "11 - TIMERAQEP"]
    TIMERAQEP = 11,
    #[doc = "12 - TIMERBQEP"]
    TIMERBQEP = 12,
    #[doc = "13 - TIMERCQEP"]
    TIMERCQEP = 13,
    #[doc = "14 - TIMERDQEP"]
    TIMERDQEP = 14,
    #[doc = "15 - GPIOA"]
    GPIOA = 15,
    #[doc = "16 - GPIOB"]
    GPIOB = 16,
    #[doc = "17 - GPIOC"]
    GPIOC = 17,
    #[doc = "18 - GPIOD"]
    GPIOD = 18,
    #[doc = "19 - GPIOE"]
    GPIOE = 19,
    #[doc = "20 - GPIOF"]
    GPIOF = 20,
    #[doc = "21 - GPIOG"]
    GPIOG = 21,
    #[doc = "22 - I2C"]
    I2C = 22,
    #[doc = "23 - USARTA"]
    USARTA = 23,
    #[doc = "24 - USARTB"]
    USARTB = 24,
    #[doc = "25 - USARTC"]
    USARTC = 25,
    #[doc = "26 - USARTD"]
    USARTD = 26,
    #[doc = "27 - CAN"]
    CAN = 27,
    #[doc = "28 - GPTIMERA"]
    GPTIMERA = 28,
    #[doc = "29 - GPTIMERB"]
    GPTIMERB = 29,
    #[doc = "30 - SCC"]
    SCC = 30,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "2.5 MSPS, 12-bit SAR ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
#[doc = "2.5 MSPS, 12-bit SAR ADC"]
pub mod adc;
#[doc = "I2C"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C").finish()
    }
}
#[doc = "I2C"]
pub mod i2c;
#[doc = "USART A"]
pub struct USARTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USARTA {}
impl USARTA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usarta::RegisterBlock = 0x4002_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usarta::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USARTA {
    type Target = usarta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USARTA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USARTA").finish()
    }
}
#[doc = "USART A"]
pub mod usarta;
#[doc = "USART B"]
pub struct USARTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USARTB {}
impl USARTB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usartb::RegisterBlock = 0x4003_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usartb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USARTB {
    type Target = usartb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USARTB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USARTB").finish()
    }
}
#[doc = "USART B"]
pub mod usartb;
#[doc = "USART C"]
pub struct USARTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USARTC {}
impl USARTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usartc::RegisterBlock = 0x4004_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usartc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USARTC {
    type Target = usartc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USARTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USARTC").finish()
    }
}
#[doc = "USART C"]
pub mod usartc;
#[doc = "USART D"]
pub struct USARTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USARTD {}
impl USARTD {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usartd::RegisterBlock = 0x4005_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usartd::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USARTD {
    type Target = usartd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USARTD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USARTD").finish()
    }
}
#[doc = "USART D"]
pub mod usartd;
#[doc = "Timer A"]
pub struct TIMERA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMERA {}
impl TIMERA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer_a::RegisterBlock = 0x4006_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_a::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMERA {
    type Target = timer_a::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMERA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERA").finish()
    }
}
#[doc = "Timer A"]
pub mod timer_a;
#[doc = "Timer B"]
pub struct TIMERB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMERB {}
impl TIMERB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer_b::RegisterBlock = 0x4007_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_b::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMERB {
    type Target = timer_b::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMERB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERB").finish()
    }
}
#[doc = "Timer B"]
pub mod timer_b;
#[doc = "Timer C"]
pub struct TIMERC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMERC {}
impl TIMERC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer_c::RegisterBlock = 0x4008_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMERC {
    type Target = timer_c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMERC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERC").finish()
    }
}
#[doc = "Timer C"]
pub mod timer_c;
#[doc = "Timer D"]
pub struct TIMERD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMERD {}
impl TIMERD {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer_d::RegisterBlock = 0x4009_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_d::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMERD {
    type Target = timer_d::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMERD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERD").finish()
    }
}
#[doc = "Timer D"]
pub mod timer_d;
#[doc = "CAN"]
pub struct CAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN {}
impl CAN {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const can::RegisterBlock = 0x400a_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CAN {
    type Target = can::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CAN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN").finish()
    }
}
#[doc = "CAN"]
pub mod can;
#[doc = "General Purpose Timer A (GPTimer A)"]
pub struct GPTIMERA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTIMERA {}
impl GPTIMERA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gptimera::RegisterBlock = 0x400b_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gptimera::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPTIMERA {
    type Target = gptimera::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPTIMERA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPTIMERA").finish()
    }
}
#[doc = "General Purpose Timer A (GPTimer A)"]
pub mod gptimera;
#[doc = "General Purpose Timer B (GPTimer B)"]
pub struct GPTIMERB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTIMERB {}
impl GPTIMERB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gptimerb::RegisterBlock = 0x400c_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gptimerb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPTIMERB {
    type Target = gptimerb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPTIMERB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPTIMERB").finish()
    }
}
#[doc = "General Purpose Timer B (GPTimer B)"]
pub mod gptimerb;
#[doc = "Memory Controller"]
pub struct MEMCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MEMCTL {}
impl MEMCTL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const memctl::RegisterBlock = 0x400d_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const memctl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MEMCTL {
    type Target = memctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MEMCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMCTL").finish()
    }
}
#[doc = "Memory Controller"]
pub mod memctl;
#[doc = "System and Clock Control"]
pub struct SCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCC {}
impl SCC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const scc::RegisterBlock = 0x400d_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SCC {
    type Target = scc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCC").finish()
    }
}
#[doc = "System and Clock Control"]
pub mod scc;
#[doc = "Windowed Watchdog Timer"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wwdt::RegisterBlock = 0x400d_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WWDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WWDT").finish()
    }
}
#[doc = "Windowed Watchdog Timer"]
pub mod wwdt;
#[doc = "Real-Time Clock (RTC) with Calendar"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x400d_0c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
#[doc = "Real-Time Clock (RTC) with Calendar"]
pub mod rtc;
#[doc = "CRC"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crc::RegisterBlock = 0x400d_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC").finish()
    }
}
#[doc = "CRC"]
pub mod crc;
#[doc = "GPIO A"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpioa::RegisterBlock = 0x400d_1400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOA").finish()
    }
}
#[doc = "GPIO A"]
pub mod gpioa;
#[doc = "GPIO B"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiob::RegisterBlock = 0x400d_1800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOB").finish()
    }
}
#[doc = "GPIO B"]
pub mod gpiob;
#[doc = "GPIO C"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpioc::RegisterBlock = 0x400d_1c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOC").finish()
    }
}
#[doc = "GPIO C"]
pub mod gpioc;
#[doc = "GPIO D"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiod::RegisterBlock = 0x400d_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOD {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOD").finish()
    }
}
#[doc = "GPIO D"]
pub mod gpiod;
#[doc = "GPIO E"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpioe::RegisterBlock = 0x400d_2400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioe::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOE {
    type Target = gpioe::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOE").finish()
    }
}
#[doc = "GPIO E"]
pub mod gpioe;
#[doc = "GPIO F"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiof::RegisterBlock = 0x400d_2800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiof::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOF {
    type Target = gpiof::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOF").finish()
    }
}
#[doc = "GPIO F"]
pub mod gpiof;
#[doc = "GPIO G"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiog::RegisterBlock = 0x400d_2c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiog::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOG {
    type Target = gpiog::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOG").finish()
    }
}
#[doc = "GPIO G"]
pub mod gpiog;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "USARTA"]
    pub USARTA: USARTA,
    #[doc = "USARTB"]
    pub USARTB: USARTB,
    #[doc = "USARTC"]
    pub USARTC: USARTC,
    #[doc = "USARTD"]
    pub USARTD: USARTD,
    #[doc = "TIMERA"]
    pub TIMERA: TIMERA,
    #[doc = "TIMERB"]
    pub TIMERB: TIMERB,
    #[doc = "TIMERC"]
    pub TIMERC: TIMERC,
    #[doc = "TIMERD"]
    pub TIMERD: TIMERD,
    #[doc = "CAN"]
    pub CAN: CAN,
    #[doc = "GPTIMERA"]
    pub GPTIMERA: GPTIMERA,
    #[doc = "GPTIMERB"]
    pub GPTIMERB: GPTIMERB,
    #[doc = "MEMCTL"]
    pub MEMCTL: MEMCTL,
    #[doc = "SCC"]
    pub SCC: SCC,
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOG"]
    pub GPIOG: GPIOG,
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
            I2C: I2C {
                _marker: PhantomData,
            },
            USARTA: USARTA {
                _marker: PhantomData,
            },
            USARTB: USARTB {
                _marker: PhantomData,
            },
            USARTC: USARTC {
                _marker: PhantomData,
            },
            USARTD: USARTD {
                _marker: PhantomData,
            },
            TIMERA: TIMERA {
                _marker: PhantomData,
            },
            TIMERB: TIMERB {
                _marker: PhantomData,
            },
            TIMERC: TIMERC {
                _marker: PhantomData,
            },
            TIMERD: TIMERD {
                _marker: PhantomData,
            },
            CAN: CAN {
                _marker: PhantomData,
            },
            GPTIMERA: GPTIMERA {
                _marker: PhantomData,
            },
            GPTIMERB: GPTIMERB {
                _marker: PhantomData,
            },
            MEMCTL: MEMCTL {
                _marker: PhantomData,
            },
            SCC: SCC {
                _marker: PhantomData,
            },
            WWDT: WWDT {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            CRC: CRC {
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
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
        }
    }
}