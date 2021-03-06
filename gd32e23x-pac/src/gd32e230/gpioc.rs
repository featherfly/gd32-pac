#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port control register"]
    pub ctl: CTL,
    #[doc = "0x04 - GPIO port output type register"]
    pub omode: OMODE,
    #[doc = "0x08 - GPIO port output speed register"]
    pub ospd: OSPD,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub pud: PUD,
    #[doc = "0x10 - GPIO port input data register"]
    pub istat: ISTAT,
    #[doc = "0x14 - GPIO port output data register"]
    pub octl: OCTL,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub bop: BOP,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - GPIO alternate function low register"]
    pub afsel0: AFSEL0,
    #[doc = "0x24 - GPIO alternate function register 1"]
    pub afsel1: AFSEL1,
    #[doc = "0x28 - Port bit reset register"]
    pub bc: BC,
    #[doc = "0x2c - Port bit toggle register"]
    pub tg: TG,
}
#[doc = "GPIO port control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "GPIO port control register"]
pub mod ctl;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [omode](omode) module"]
pub type OMODE = crate::Reg<u32, _OMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OMODE;
#[doc = "`read()` method returns [omode::R](omode::R) reader structure"]
impl crate::Readable for OMODE {}
#[doc = "`write(|w| ..)` method takes [omode::W](omode::W) writer structure"]
impl crate::Writable for OMODE {}
#[doc = "GPIO port output type register"]
pub mod omode;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospd](ospd) module"]
pub type OSPD = crate::Reg<u32, _OSPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSPD;
#[doc = "`read()` method returns [ospd::R](ospd::R) reader structure"]
impl crate::Readable for OSPD {}
#[doc = "`write(|w| ..)` method takes [ospd::W](ospd::W) writer structure"]
impl crate::Writable for OSPD {}
#[doc = "GPIO port output speed register"]
pub mod ospd;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pud](pud) module"]
pub type PUD = crate::Reg<u32, _PUD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUD;
#[doc = "`read()` method returns [pud::R](pud::R) reader structure"]
impl crate::Readable for PUD {}
#[doc = "`write(|w| ..)` method takes [pud::W](pud::W) writer structure"]
impl crate::Writable for PUD {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod pud;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istat](istat) module"]
pub type ISTAT = crate::Reg<u32, _ISTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISTAT;
#[doc = "`read()` method returns [istat::R](istat::R) reader structure"]
impl crate::Readable for ISTAT {}
#[doc = "GPIO port input data register"]
pub mod istat;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octl](octl) module"]
pub type OCTL = crate::Reg<u32, _OCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCTL;
#[doc = "`read()` method returns [octl::R](octl::R) reader structure"]
impl crate::Readable for OCTL {}
#[doc = "`write(|w| ..)` method takes [octl::W](octl::W) writer structure"]
impl crate::Writable for OCTL {}
#[doc = "GPIO port output data register"]
pub mod octl;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bop](bop) module"]
pub type BOP = crate::Reg<u32, _BOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOP;
#[doc = "`write(|w| ..)` method takes [bop::W](bop::W) writer structure"]
impl crate::Writable for BOP {}
#[doc = "GPIO port bit set/reset register"]
pub mod bop;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afsel0](afsel0) module"]
pub type AFSEL0 = crate::Reg<u32, _AFSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFSEL0;
#[doc = "`read()` method returns [afsel0::R](afsel0::R) reader structure"]
impl crate::Readable for AFSEL0 {}
#[doc = "`write(|w| ..)` method takes [afsel0::W](afsel0::W) writer structure"]
impl crate::Writable for AFSEL0 {}
#[doc = "GPIO alternate function low register"]
pub mod afsel0;
#[doc = "GPIO alternate function register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afsel1](afsel1) module"]
pub type AFSEL1 = crate::Reg<u32, _AFSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFSEL1;
#[doc = "`read()` method returns [afsel1::R](afsel1::R) reader structure"]
impl crate::Readable for AFSEL1 {}
#[doc = "`write(|w| ..)` method takes [afsel1::W](afsel1::W) writer structure"]
impl crate::Writable for AFSEL1 {}
#[doc = "GPIO alternate function register 1"]
pub mod afsel1;
#[doc = "Port bit reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bc](bc) module"]
pub type BC = crate::Reg<u32, _BC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BC;
#[doc = "`write(|w| ..)` method takes [bc::W](bc::W) writer structure"]
impl crate::Writable for BC {}
#[doc = "Port bit reset register"]
pub mod bc;
#[doc = "Port bit toggle register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tg](tg) module"]
pub type TG = crate::Reg<u32, _TG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TG;
#[doc = "`write(|w| ..)` method takes [tg::W](tg::W) writer structure"]
impl crate::Writable for TG {}
#[doc = "Port bit toggle register"]
pub mod tg;
