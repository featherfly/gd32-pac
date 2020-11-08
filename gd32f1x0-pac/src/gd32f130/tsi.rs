# [ doc = r"Register block" ] # [ repr ( C ) ] pub struct RegisterBlock { # [ doc = "0x00 - control register" ] pub ctl : CTL , # [ doc = "0x04 - interrupt enable register" ] pub inten : INTEN , # [ doc = "0x08 - interrupt flag clear register" ] pub intc : INTC , # [ doc = "0x0c - interrupt flag register" ] pub intf : INTF , # [ doc = "0x10 - Pin hysteresis mode register" ] pub phm : PHM , _reserved5 : [ u8 ; 4usize ] , # [ doc = "0x18 - I/O analog switch register" ] pub asw : ASW , _reserved6 : [ u8 ; 4usize ] , # [ doc = "0x20 - I/O sample configuration register" ] pub sampcfg : SAMPCFG , _reserved7 : [ u8 ; 4usize ] , # [ doc = "0x28 - I/O channel configuration register" ] pub chcfg : CHCFG , _reserved8 : [ u8 ; 4usize ] , # [ doc = "0x30 - I/O group control register" ] pub gctl : GCTL , # [ doc = "0x34 - I/O group x cycle number register" ] pub g0cycn : G0CYCN , # [ doc = "0x38 - I/O group x cycle number register" ] pub g1cycn : G1CYCN , # [ doc = "0x3c - I/O group x cycle number register" ] pub g2cycn : G2CYCN , # [ doc = "0x40 - I/O group x cycle number register" ] pub g3cycn : G3CYCN , # [ doc = "0x44 - I/O group x cycle number register" ] pub g4cycn : G4CYCN , # [ doc = "0x48 - I/O group x cycle number register" ] pub g5cycn : G5CYCN , } # [ doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module" ] pub type CTL = crate :: Reg < u32 , _CTL > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _CTL ; # [ doc = "`read()` method returns [ctl::R](ctl::R) reader structure" ] impl crate :: Readable for CTL { } # [ doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure" ] impl crate :: Writable for CTL { } # [ doc = "control register" ] pub mod ctl ; # [ doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module" ] pub type INTEN = crate :: Reg < u32 , _INTEN > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _INTEN ; # [ doc = "`read()` method returns [inten::R](inten::R) reader structure" ] impl crate :: Readable for INTEN { } # [ doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure" ] impl crate :: Writable for INTEN { } # [ doc = "interrupt enable register" ] pub mod inten ; # [ doc = "interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](intc) module" ] pub type INTC = crate :: Reg < u32 , _INTC > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _INTC ; # [ doc = "`read()` method returns [intc::R](intc::R) reader structure" ] impl crate :: Readable for INTC { } # [ doc = "`write(|w| ..)` method takes [intc::W](intc::W) writer structure" ] impl crate :: Writable for INTC { } # [ doc = "interrupt flag clear register" ] pub mod intc ; # [ doc = "interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](intf) module" ] pub type INTF = crate :: Reg < u32 , _INTF > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _INTF ; # [ doc = "`read()` method returns [intf::R](intf::R) reader structure" ] impl crate :: Readable for INTF { } # [ doc = "`write(|w| ..)` method takes [intf::W](intf::W) writer structure" ] impl crate :: Writable for INTF { } # [ doc = "interrupt flag register" ] pub mod intf ; # [ doc = "Pin hysteresis mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phm](phm) module" ] pub type PHM = crate :: Reg < u32 , _PHM > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _PHM ; # [ doc = "`read()` method returns [phm::R](phm::R) reader structure" ] impl crate :: Readable for PHM { } # [ doc = "`write(|w| ..)` method takes [phm::W](phm::W) writer structure" ] impl crate :: Writable for PHM { } # [ doc = "Pin hysteresis mode register" ] pub mod phm ; # [ doc = "I/O analog switch register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asw](asw) module" ] pub type ASW = crate :: Reg < u32 , _ASW > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _ASW ; # [ doc = "`read()` method returns [asw::R](asw::R) reader structure" ] impl crate :: Readable for ASW { } # [ doc = "`write(|w| ..)` method takes [asw::W](asw::W) writer structure" ] impl crate :: Writable for ASW { } # [ doc = "I/O analog switch register" ] pub mod asw ; # [ doc = "I/O sample configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampcfg](sampcfg) module" ] pub type SAMPCFG = crate :: Reg < u32 , _SAMPCFG > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _SAMPCFG ; # [ doc = "`read()` method returns [sampcfg::R](sampcfg::R) reader structure" ] impl crate :: Readable for SAMPCFG { } # [ doc = "`write(|w| ..)` method takes [sampcfg::W](sampcfg::W) writer structure" ] impl crate :: Writable for SAMPCFG { } # [ doc = "I/O sample configuration register" ] pub mod sampcfg ; # [ doc = "I/O channel configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chcfg](chcfg) module" ] pub type CHCFG = crate :: Reg < u32 , _CHCFG > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _CHCFG ; # [ doc = "`read()` method returns [chcfg::R](chcfg::R) reader structure" ] impl crate :: Readable for CHCFG { } # [ doc = "`write(|w| ..)` method takes [chcfg::W](chcfg::W) writer structure" ] impl crate :: Writable for CHCFG { } # [ doc = "I/O channel configuration register" ] pub mod chcfg ; # [ doc = "I/O group control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gctl](gctl) module" ] pub type GCTL = crate :: Reg < u32 , _GCTL > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _GCTL ; # [ doc = "`read()` method returns [gctl::R](gctl::R) reader structure" ] impl crate :: Readable for GCTL { } # [ doc = "`write(|w| ..)` method takes [gctl::W](gctl::W) writer structure" ] impl crate :: Writable for GCTL { } # [ doc = "I/O group control register" ] pub mod gctl ; # [ doc = "I/O group x cycle number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [g0cycn](g0cycn) module" ] pub type G0CYCN = crate :: Reg < u32 , _G0CYCN > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _G0CYCN ; # [ doc = "`read()` method returns [g0cycn::R](g0cycn::R) reader structure" ] impl crate :: Readable for G0CYCN { } # [ doc = "I/O group x cycle number register" ] pub mod g0cycn ; # [ doc = "I/O group x cycle number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [g1cycn](g1cycn) module" ] pub type G1CYCN = crate :: Reg < u32 , _G1CYCN > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _G1CYCN ; # [ doc = "`read()` method returns [g1cycn::R](g1cycn::R) reader structure" ] impl crate :: Readable for G1CYCN { } # [ doc = "I/O group x cycle number register" ] pub mod g1cycn ; # [ doc = "I/O group x cycle number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [g2cycn](g2cycn) module" ] pub type G2CYCN = crate :: Reg < u32 , _G2CYCN > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _G2CYCN ; # [ doc = "`read()` method returns [g2cycn::R](g2cycn::R) reader structure" ] impl crate :: Readable for G2CYCN { } # [ doc = "I/O group x cycle number register" ] pub mod g2cycn ; # [ doc = "I/O group x cycle number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [g3cycn](g3cycn) module" ] pub type G3CYCN = crate :: Reg < u32 , _G3CYCN > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _G3CYCN ; # [ doc = "`read()` method returns [g3cycn::R](g3cycn::R) reader structure" ] impl crate :: Readable for G3CYCN { } # [ doc = "I/O group x cycle number register" ] pub mod g3cycn ; # [ doc = "I/O group x cycle number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [g4cycn](g4cycn) module" ] pub type G4CYCN = crate :: Reg < u32 , _G4CYCN > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _G4CYCN ; # [ doc = "`read()` method returns [g4cycn::R](g4cycn::R) reader structure" ] impl crate :: Readable for G4CYCN { } # [ doc = "I/O group x cycle number register" ] pub mod g4cycn ; # [ doc = "I/O group x cycle number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [g5cycn](g5cycn) module" ] pub type G5CYCN = crate :: Reg < u32 , _G5CYCN > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _G5CYCN ; # [ doc = "`read()` method returns [g5cycn::R](g5cycn::R) reader structure" ] impl crate :: Readable for G5CYCN { } # [ doc = "I/O group x cycle number register" ] pub mod g5cycn ;