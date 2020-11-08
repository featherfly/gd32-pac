#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub time: TIME,
    #[doc = "0x04 - date register"]
    pub date: DATE,
    #[doc = "0x08 - control register"]
    pub ctl: CTL,
    #[doc = "0x0c - initialization and status register"]
    pub stat: STAT,
    #[doc = "0x10 - prescaler register"]
    pub psc: PSC,
    _reserved5: [u8; 8usize],
    #[doc = "0x1c - alarm A register"]
    pub alrm0td: ALRM0TD,
    _reserved6: [u8; 4usize],
    #[doc = "0x24 - write protection register"]
    pub wpk: WPK,
    #[doc = "0x28 - sub second register"]
    pub ss: SS,
    #[doc = "0x2c - shift control register"]
    pub shiftctl: SHIFTCTL,
    #[doc = "0x30 - timestamp time register"]
    pub tts: TTS,
    #[doc = "0x34 - Date of time stamp register"]
    pub dts: DTS,
    #[doc = "0x38 - time-stamp sub second register"]
    pub ssts: SSTS,
    #[doc = "0x3c - High resolution frequency compensation register"]
    pub hrfc: HRFC,
    #[doc = "0x40 - tamper and alternate function configuration register"]
    pub tamp: TAMP,
    #[doc = "0x44 - alarm 0 sub second register"]
    pub alrm0ss: ALRM0SS,
    _reserved15: [u8; 8usize],
    #[doc = "0x50 - backup register"]
    pub bkp0: BKP0,
    #[doc = "0x54 - backup register"]
    pub bkp1: BKP1,
    #[doc = "0x58 - backup register"]
    pub bkp2: BKP2,
    #[doc = "0x5c - backup register"]
    pub bkp3: BKP3,
    #[doc = "0x60 - backup register"]
    pub bkp4: BKP4,
}
#[doc = "time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time](time) module"]
pub type TIME = crate::Reg<u32, _TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME;
#[doc = "`read()` method returns [time::R](time::R) reader structure"]
impl crate::Readable for TIME {}
#[doc = "`write(|w| ..)` method takes [time::W](time::W) writer structure"]
impl crate::Writable for TIME {}
#[doc = "time register"]
pub mod time;
#[doc = "date register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "date register"]
pub mod date;
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "control register"]
pub mod ctl;
#[doc = "initialization and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "initialization and status register"]
pub mod stat;
#[doc = "prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psc](psc) module"]
pub type PSC = crate::Reg<u32, _PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSC;
#[doc = "`read()` method returns [psc::R](psc::R) reader structure"]
impl crate::Readable for PSC {}
#[doc = "`write(|w| ..)` method takes [psc::W](psc::W) writer structure"]
impl crate::Writable for PSC {}
#[doc = "prescaler register"]
pub mod psc;
#[doc = "alarm A register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrm0td](alrm0td) module"]
pub type ALRM0TD = crate::Reg<u32, _ALRM0TD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRM0TD;
#[doc = "`read()` method returns [alrm0td::R](alrm0td::R) reader structure"]
impl crate::Readable for ALRM0TD {}
#[doc = "`write(|w| ..)` method takes [alrm0td::W](alrm0td::W) writer structure"]
impl crate::Writable for ALRM0TD {}
#[doc = "alarm A register"]
pub mod alrm0td;
#[doc = "write protection register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpk](wpk) module"]
pub type WPK = crate::Reg<u32, _WPK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPK;
#[doc = "`write(|w| ..)` method takes [wpk::W](wpk::W) writer structure"]
impl crate::Writable for WPK {}
#[doc = "write protection register"]
pub mod wpk;
#[doc = "sub second register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ss](ss) module"]
pub type SS = crate::Reg<u32, _SS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SS;
#[doc = "`read()` method returns [ss::R](ss::R) reader structure"]
impl crate::Readable for SS {}
#[doc = "sub second register"]
pub mod ss;
#[doc = "shift control register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftctl](shiftctl) module"]
pub type SHIFTCTL = crate::Reg<u32, _SHIFTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCTL;
#[doc = "`write(|w| ..)` method takes [shiftctl::W](shiftctl::W) writer structure"]
impl crate::Writable for SHIFTCTL {}
#[doc = "shift control register"]
pub mod shiftctl;
#[doc = "timestamp time register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tts](tts) module"]
pub type TTS = crate::Reg<u32, _TTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTS;
#[doc = "`read()` method returns [tts::R](tts::R) reader structure"]
impl crate::Readable for TTS {}
#[doc = "timestamp time register"]
pub mod tts;
#[doc = "Date of time stamp register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts](dts) module"]
pub type DTS = crate::Reg<u32, _DTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTS;
#[doc = "`read()` method returns [dts::R](dts::R) reader structure"]
impl crate::Readable for DTS {}
#[doc = "Date of time stamp register"]
pub mod dts;
#[doc = "time-stamp sub second register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssts](ssts) module"]
pub type SSTS = crate::Reg<u32, _SSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSTS;
#[doc = "`read()` method returns [ssts::R](ssts::R) reader structure"]
impl crate::Readable for SSTS {}
#[doc = "time-stamp sub second register"]
pub mod ssts;
#[doc = "High resolution frequency compensation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrfc](hrfc) module"]
pub type HRFC = crate::Reg<u32, _HRFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRFC;
#[doc = "`read()` method returns [hrfc::R](hrfc::R) reader structure"]
impl crate::Readable for HRFC {}
#[doc = "`write(|w| ..)` method takes [hrfc::W](hrfc::W) writer structure"]
impl crate::Writable for HRFC {}
#[doc = "High resolution frequency compensation register"]
pub mod hrfc;
#[doc = "tamper and alternate function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp](tamp) module"]
pub type TAMP = crate::Reg<u32, _TAMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMP;
#[doc = "`read()` method returns [tamp::R](tamp::R) reader structure"]
impl crate::Readable for TAMP {}
#[doc = "`write(|w| ..)` method takes [tamp::W](tamp::W) writer structure"]
impl crate::Writable for TAMP {}
#[doc = "tamper and alternate function configuration register"]
pub mod tamp;
#[doc = "alarm 0 sub second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrm0ss](alrm0ss) module"]
pub type ALRM0SS = crate::Reg<u32, _ALRM0SS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRM0SS;
#[doc = "`read()` method returns [alrm0ss::R](alrm0ss::R) reader structure"]
impl crate::Readable for ALRM0SS {}
#[doc = "`write(|w| ..)` method takes [alrm0ss::W](alrm0ss::W) writer structure"]
impl crate::Writable for ALRM0SS {}
#[doc = "alarm 0 sub second register"]
pub mod alrm0ss;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp0](bkp0) module"]
pub type BKP0 = crate::Reg<u32, _BKP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP0;
#[doc = "`read()` method returns [bkp0::R](bkp0::R) reader structure"]
impl crate::Readable for BKP0 {}
#[doc = "`write(|w| ..)` method takes [bkp0::W](bkp0::W) writer structure"]
impl crate::Writable for BKP0 {}
#[doc = "backup register"]
pub mod bkp0;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp1](bkp1) module"]
pub type BKP1 = crate::Reg<u32, _BKP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP1;
#[doc = "`read()` method returns [bkp1::R](bkp1::R) reader structure"]
impl crate::Readable for BKP1 {}
#[doc = "`write(|w| ..)` method takes [bkp1::W](bkp1::W) writer structure"]
impl crate::Writable for BKP1 {}
#[doc = "backup register"]
pub mod bkp1;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp2](bkp2) module"]
pub type BKP2 = crate::Reg<u32, _BKP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP2;
#[doc = "`read()` method returns [bkp2::R](bkp2::R) reader structure"]
impl crate::Readable for BKP2 {}
#[doc = "`write(|w| ..)` method takes [bkp2::W](bkp2::W) writer structure"]
impl crate::Writable for BKP2 {}
#[doc = "backup register"]
pub mod bkp2;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp3](bkp3) module"]
pub type BKP3 = crate::Reg<u32, _BKP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP3;
#[doc = "`read()` method returns [bkp3::R](bkp3::R) reader structure"]
impl crate::Readable for BKP3 {}
#[doc = "`write(|w| ..)` method takes [bkp3::W](bkp3::W) writer structure"]
impl crate::Writable for BKP3 {}
#[doc = "backup register"]
pub mod bkp3;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp4](bkp4) module"]
pub type BKP4 = crate::Reg<u32, _BKP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP4;
#[doc = "`read()` method returns [bkp4::R](bkp4::R) reader structure"]
impl crate::Readable for BKP4 {}
#[doc = "`write(|w| ..)` method takes [bkp4::W](bkp4::W) writer structure"]
impl crate::Writable for BKP4 {}
#[doc = "backup register"]
pub mod bkp4;
