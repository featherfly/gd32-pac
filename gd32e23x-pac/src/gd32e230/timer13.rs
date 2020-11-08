#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub ctl0: CTL0,
    _reserved1: [u8; 8usize],
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dmainten: DMAINTEN,
    #[doc = "0x10 - interrupt flag register"]
    pub intf: INTF,
    #[doc = "0x14 - event generation register"]
    pub swevg: SWEVG,
    _reserved_4_chctl0: [u8; 4usize],
    _reserved5: [u8; 4usize],
    #[doc = "0x20 - capture/compare enable register"]
    pub chctl2: CHCTL2,
    #[doc = "0x24 - counter"]
    pub cnt: CNT,
    #[doc = "0x28 - prescaler"]
    pub psc: PSC,
    #[doc = "0x2c - auto-reload register"]
    pub car: CAR,
    _reserved9: [u8; 4usize],
    #[doc = "0x34 - capture/compare register 0"]
    pub ch0cv: CH0CV,
    _reserved10: [u8; 24usize],
    #[doc = "0x50 - channel input remap register"]
    pub irmp: IRMP,
    _reserved11: [u8; 168usize],
    #[doc = "0xfc - configuration register"]
    pub cfg: CFG,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register (input mode)"]
    #[inline(always)]
    pub fn chctl0_input(&self) -> &CHCTL0_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CHCTL0_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register (input mode)"]
    #[inline(always)]
    pub fn chctl0_input_mut(&self) -> &mut CHCTL0_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CHCTL0_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register (output mode)"]
    #[inline(always)]
    pub fn chctl0_output(&self) -> &CHCTL0_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CHCTL0_OUTPUT) }
    }
    #[doc = "0x18 - capture/compare mode register (output mode)"]
    #[inline(always)]
    pub fn chctl0_output_mut(&self) -> &mut CHCTL0_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CHCTL0_OUTPUT) }
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u32, _CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL0;
#[doc = "`read()` method returns [ctl0::R](ctl0::R) reader structure"]
impl crate::Readable for CTL0 {}
#[doc = "`write(|w| ..)` method takes [ctl0::W](ctl0::W) writer structure"]
impl crate::Writable for CTL0 {}
#[doc = "control register 1"]
pub mod ctl0;
#[doc = "DMA/Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmainten](dmainten) module"]
pub type DMAINTEN = crate::Reg<u32, _DMAINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAINTEN;
#[doc = "`read()` method returns [dmainten::R](dmainten::R) reader structure"]
impl crate::Readable for DMAINTEN {}
#[doc = "`write(|w| ..)` method takes [dmainten::W](dmainten::W) writer structure"]
impl crate::Writable for DMAINTEN {}
#[doc = "DMA/Interrupt enable register"]
pub mod dmainten;
#[doc = "interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](intf) module"]
pub type INTF = crate::Reg<u32, _INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF;
#[doc = "`read()` method returns [intf::R](intf::R) reader structure"]
impl crate::Readable for INTF {}
#[doc = "`write(|w| ..)` method takes [intf::W](intf::W) writer structure"]
impl crate::Writable for INTF {}
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevg](swevg) module"]
pub type SWEVG = crate::Reg<u32, _SWEVG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWEVG;
#[doc = "`write(|w| ..)` method takes [swevg::W](swevg::W) writer structure"]
impl crate::Writable for SWEVG {}
#[doc = "event generation register"]
pub mod swevg;
#[doc = "capture/compare mode register (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl0_output](chctl0_output) module"]
pub type CHCTL0_OUTPUT = crate::Reg<u32, _CHCTL0_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTL0_OUTPUT;
#[doc = "`read()` method returns [chctl0_output::R](chctl0_output::R) reader structure"]
impl crate::Readable for CHCTL0_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [chctl0_output::W](chctl0_output::W) writer structure"]
impl crate::Writable for CHCTL0_OUTPUT {}
#[doc = "capture/compare mode register (output mode)"]
pub mod chctl0_output;
#[doc = "capture/compare mode register (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl0_input](chctl0_input) module"]
pub type CHCTL0_INPUT = crate::Reg<u32, _CHCTL0_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTL0_INPUT;
#[doc = "`read()` method returns [chctl0_input::R](chctl0_input::R) reader structure"]
impl crate::Readable for CHCTL0_INPUT {}
#[doc = "`write(|w| ..)` method takes [chctl0_input::W](chctl0_input::W) writer structure"]
impl crate::Writable for CHCTL0_INPUT {}
#[doc = "capture/compare mode register (input mode)"]
pub mod chctl0_input;
#[doc = "capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl2](chctl2) module"]
pub type CHCTL2 = crate::Reg<u32, _CHCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTL2;
#[doc = "`read()` method returns [chctl2::R](chctl2::R) reader structure"]
impl crate::Readable for CHCTL2 {}
#[doc = "`write(|w| ..)` method takes [chctl2::W](chctl2::W) writer structure"]
impl crate::Writable for CHCTL2 {}
#[doc = "capture/compare enable register"]
pub mod chctl2;
#[doc = "counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "counter"]
pub mod cnt;
#[doc = "prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psc](psc) module"]
pub type PSC = crate::Reg<u32, _PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSC;
#[doc = "`read()` method returns [psc::R](psc::R) reader structure"]
impl crate::Readable for PSC {}
#[doc = "`write(|w| ..)` method takes [psc::W](psc::W) writer structure"]
impl crate::Writable for PSC {}
#[doc = "prescaler"]
pub mod psc;
#[doc = "auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [car](car) module"]
pub type CAR = crate::Reg<u32, _CAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAR;
#[doc = "`read()` method returns [car::R](car::R) reader structure"]
impl crate::Readable for CAR {}
#[doc = "`write(|w| ..)` method takes [car::W](car::W) writer structure"]
impl crate::Writable for CAR {}
#[doc = "auto-reload register"]
pub mod car;
#[doc = "capture/compare register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0cv](ch0cv) module"]
pub type CH0CV = crate::Reg<u32, _CH0CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CV;
#[doc = "`read()` method returns [ch0cv::R](ch0cv::R) reader structure"]
impl crate::Readable for CH0CV {}
#[doc = "`write(|w| ..)` method takes [ch0cv::W](ch0cv::W) writer structure"]
impl crate::Writable for CH0CV {}
#[doc = "capture/compare register 0"]
pub mod ch0cv;
#[doc = "channel input remap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irmp](irmp) module"]
pub type IRMP = crate::Reg<u32, _IRMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRMP;
#[doc = "`read()` method returns [irmp::R](irmp::R) reader structure"]
impl crate::Readable for IRMP {}
#[doc = "`write(|w| ..)` method takes [irmp::W](irmp::W) writer structure"]
impl crate::Writable for IRMP {}
#[doc = "channel input remap register"]
pub mod irmp;
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "configuration register"]
pub mod cfg;
