#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - wait state register"]
    pub ws: WS,
    #[doc = "0x04 - Unlock key register"]
    pub key: KEY,
    #[doc = "0x08 - Option byte unlock key register"]
    pub obkey: OBKEY,
    #[doc = "0x0c - Status register"]
    pub stat: STAT,
    #[doc = "0x10 - Control register"]
    pub ctl: CTL,
    #[doc = "0x14 - Address register"]
    pub addr: ADDR,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - Option byte control register"]
    pub obstat: OBSTAT,
    #[doc = "0x20 - Erase/Program Protection register"]
    pub wp: WP,
    _reserved8: [u8; 220usize],
    #[doc = "0x100 - Product ID register"]
    pub pid: PID,
}
#[doc = "wait state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ws](ws) module"]
pub type WS = crate::Reg<u32, _WS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WS;
#[doc = "`read()` method returns [ws::R](ws::R) reader structure"]
impl crate::Readable for WS {}
#[doc = "`write(|w| ..)` method takes [ws::W](ws::W) writer structure"]
impl crate::Writable for WS {}
#[doc = "wait state register"]
pub mod ws;
#[doc = "Unlock key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key](key) module"]
pub type KEY = crate::Reg<u32, _KEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY;
#[doc = "`write(|w| ..)` method takes [key::W](key::W) writer structure"]
impl crate::Writable for KEY {}
#[doc = "Unlock key register"]
pub mod key;
#[doc = "Option byte unlock key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obkey](obkey) module"]
pub type OBKEY = crate::Reg<u32, _OBKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBKEY;
#[doc = "`write(|w| ..)` method takes [obkey::W](obkey::W) writer structure"]
impl crate::Writable for OBKEY {}
#[doc = "Option byte unlock key register"]
pub mod obkey;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "Status register"]
pub mod stat;
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control register"]
pub mod ctl;
#[doc = "Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "Address register"]
pub mod addr;
#[doc = "Option byte control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obstat](obstat) module"]
pub type OBSTAT = crate::Reg<u32, _OBSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSTAT;
#[doc = "`read()` method returns [obstat::R](obstat::R) reader structure"]
impl crate::Readable for OBSTAT {}
#[doc = "Option byte control register"]
pub mod obstat;
#[doc = "Erase/Program Protection register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wp](wp) module"]
pub type WP = crate::Reg<u32, _WP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WP;
#[doc = "`read()` method returns [wp::R](wp::R) reader structure"]
impl crate::Readable for WP {}
#[doc = "Erase/Program Protection register"]
pub mod wp;
#[doc = "Product ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid](pid) module"]
pub type PID = crate::Reg<u32, _PID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID;
#[doc = "`read()` method returns [pid::R](pid::R) reader structure"]
impl crate::Readable for PID {}
#[doc = "Product ID register"]
pub mod pid;
