#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Selector Configuration"]
    pub ins: INS,
    #[doc = "0x04 - Connection Matrix Control"]
    pub cmc: CMC,
    #[doc = "0x08 - Slice Timer Status"]
    pub tcst: TCST,
    #[doc = "0x0c - Slice Timer Run Set"]
    pub tcset: TCSET,
    #[doc = "0x10 - Slice Timer Clear"]
    pub tcclr: TCCLR,
    #[doc = "0x14 - Slice Timer Control"]
    pub tc: TC,
    #[doc = "0x18 - Passive Level Config"]
    pub psl: PSL,
    #[doc = "0x1c - Dither Config"]
    pub dit: DIT,
    #[doc = "0x20 - Dither Shadow Register"]
    pub dits: DITS,
    #[doc = "0x24 - Prescaler Control"]
    pub psc: PSC,
    #[doc = "0x28 - Floating Prescaler Control"]
    pub fpc: FPC,
    #[doc = "0x2c - Floating Prescaler Shadow"]
    pub fpcs: FPCS,
    #[doc = "0x30 - Timer Period Value"]
    pub pr: PR,
    #[doc = "0x34 - Timer Shadow Period Value"]
    pub prs: PRS,
    #[doc = "0x38 - Channel 1 Compare Value"]
    pub cr1: CR1,
    #[doc = "0x3c - Channel 1 Compare Shadow Value"]
    pub cr1s: CR1S,
    #[doc = "0x40 - Channel 2 Compare Value"]
    pub cr2: CR2,
    #[doc = "0x44 - Channel 2 Compare Shadow Value"]
    pub cr2s: CR2S,
    #[doc = "0x48 - Channel Control"]
    pub chc: CHC,
    #[doc = "0x4c - Dead Time Control"]
    pub dtc: DTC,
    #[doc = "0x50 - Channel 1 Dead Time Values"]
    pub dc1r: DC1R,
    #[doc = "0x54 - Channel 2 Dead Time Values"]
    pub dc2r: DC2R,
    _reserved22: [u8; 24usize],
    #[doc = "0x70 - Timer Value"]
    pub timer: TIMER,
    #[doc = "0x74 - Capture Register 0"]
    pub c0v: C0V,
    #[doc = "0x78 - Capture Register 1"]
    pub c1v: C1V,
    #[doc = "0x7c - Capture Register 2"]
    pub c2v: C2V,
    #[doc = "0x80 - Capture Register 3"]
    pub c3v: C3V,
    _reserved27: [u8; 28usize],
    #[doc = "0xa0 - Interrupt Status"]
    pub ints: INTS,
    #[doc = "0xa4 - Interrupt Enable Control"]
    pub inte: INTE,
    #[doc = "0xa8 - Service Request Selector"]
    pub srs: SRS,
    #[doc = "0xac - Interrupt Status Set"]
    pub sws: SWS,
    #[doc = "0xb0 - Interrupt Status Clear"]
    pub swr: SWR,
}
#[doc = "Input Selector Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ins](ins) module"]
pub type INS = crate::Reg<u32, _INS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INS;
#[doc = "`read()` method returns [ins::R](ins::R) reader structure"]
impl crate::Readable for INS {}
#[doc = "`write(|w| ..)` method takes [ins::W](ins::W) writer structure"]
impl crate::Writable for INS {}
#[doc = "Input Selector Configuration"]
pub mod ins;
#[doc = "Connection Matrix Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmc](cmc) module"]
pub type CMC = crate::Reg<u32, _CMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMC;
#[doc = "`read()` method returns [cmc::R](cmc::R) reader structure"]
impl crate::Readable for CMC {}
#[doc = "`write(|w| ..)` method takes [cmc::W](cmc::W) writer structure"]
impl crate::Writable for CMC {}
#[doc = "Connection Matrix Control"]
pub mod cmc;
#[doc = "Slice Timer Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcst](tcst) module"]
pub type TCST = crate::Reg<u32, _TCST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCST;
#[doc = "`read()` method returns [tcst::R](tcst::R) reader structure"]
impl crate::Readable for TCST {}
#[doc = "Slice Timer Status"]
pub mod tcst;
#[doc = "Slice Timer Run Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcset](tcset) module"]
pub type TCSET = crate::Reg<u32, _TCSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCSET;
#[doc = "`write(|w| ..)` method takes [tcset::W](tcset::W) writer structure"]
impl crate::Writable for TCSET {}
#[doc = "Slice Timer Run Set"]
pub mod tcset;
#[doc = "Slice Timer Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcclr](tcclr) module"]
pub type TCCLR = crate::Reg<u32, _TCCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCCLR;
#[doc = "`write(|w| ..)` method takes [tcclr::W](tcclr::W) writer structure"]
impl crate::Writable for TCCLR {}
#[doc = "Slice Timer Clear"]
pub mod tcclr;
#[doc = "Slice Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc](tc) module"]
pub type TC = crate::Reg<u32, _TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC;
#[doc = "`read()` method returns [tc::R](tc::R) reader structure"]
impl crate::Readable for TC {}
#[doc = "`write(|w| ..)` method takes [tc::W](tc::W) writer structure"]
impl crate::Writable for TC {}
#[doc = "Slice Timer Control"]
pub mod tc;
#[doc = "Passive Level Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psl](psl) module"]
pub type PSL = crate::Reg<u32, _PSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSL;
#[doc = "`read()` method returns [psl::R](psl::R) reader structure"]
impl crate::Readable for PSL {}
#[doc = "`write(|w| ..)` method takes [psl::W](psl::W) writer structure"]
impl crate::Writable for PSL {}
#[doc = "Passive Level Config"]
pub mod psl;
#[doc = "Dither Config\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dit](dit) module"]
pub type DIT = crate::Reg<u32, _DIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIT;
#[doc = "`read()` method returns [dit::R](dit::R) reader structure"]
impl crate::Readable for DIT {}
#[doc = "Dither Config"]
pub mod dit;
#[doc = "Dither Shadow Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dits](dits) module"]
pub type DITS = crate::Reg<u32, _DITS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITS;
#[doc = "`read()` method returns [dits::R](dits::R) reader structure"]
impl crate::Readable for DITS {}
#[doc = "`write(|w| ..)` method takes [dits::W](dits::W) writer structure"]
impl crate::Writable for DITS {}
#[doc = "Dither Shadow Register"]
pub mod dits;
#[doc = "Prescaler Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psc](psc) module"]
pub type PSC = crate::Reg<u32, _PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSC;
#[doc = "`read()` method returns [psc::R](psc::R) reader structure"]
impl crate::Readable for PSC {}
#[doc = "`write(|w| ..)` method takes [psc::W](psc::W) writer structure"]
impl crate::Writable for PSC {}
#[doc = "Prescaler Control"]
pub mod psc;
#[doc = "Floating Prescaler Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpc](fpc) module"]
pub type FPC = crate::Reg<u32, _FPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPC;
#[doc = "`read()` method returns [fpc::R](fpc::R) reader structure"]
impl crate::Readable for FPC {}
#[doc = "`write(|w| ..)` method takes [fpc::W](fpc::W) writer structure"]
impl crate::Writable for FPC {}
#[doc = "Floating Prescaler Control"]
pub mod fpc;
#[doc = "Floating Prescaler Shadow\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpcs](fpcs) module"]
pub type FPCS = crate::Reg<u32, _FPCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCS;
#[doc = "`read()` method returns [fpcs::R](fpcs::R) reader structure"]
impl crate::Readable for FPCS {}
#[doc = "`write(|w| ..)` method takes [fpcs::W](fpcs::W) writer structure"]
impl crate::Writable for FPCS {}
#[doc = "Floating Prescaler Shadow"]
pub mod fpcs;
#[doc = "Timer Period Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](pr) module"]
pub type PR = crate::Reg<u32, _PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR;
#[doc = "`read()` method returns [pr::R](pr::R) reader structure"]
impl crate::Readable for PR {}
#[doc = "Timer Period Value"]
pub mod pr;
#[doc = "Timer Shadow Period Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs](prs) module"]
pub type PRS = crate::Reg<u32, _PRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRS;
#[doc = "`read()` method returns [prs::R](prs::R) reader structure"]
impl crate::Readable for PRS {}
#[doc = "`write(|w| ..)` method takes [prs::W](prs::W) writer structure"]
impl crate::Writable for PRS {}
#[doc = "Timer Shadow Period Value"]
pub mod prs;
#[doc = "Channel 1 Compare Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "Channel 1 Compare Value"]
pub mod cr1;
#[doc = "Channel 1 Compare Shadow Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1s](cr1s) module"]
pub type CR1S = crate::Reg<u32, _CR1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1S;
#[doc = "`read()` method returns [cr1s::R](cr1s::R) reader structure"]
impl crate::Readable for CR1S {}
#[doc = "`write(|w| ..)` method takes [cr1s::W](cr1s::W) writer structure"]
impl crate::Writable for CR1S {}
#[doc = "Channel 1 Compare Shadow Value"]
pub mod cr1s;
#[doc = "Channel 2 Compare Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "Channel 2 Compare Value"]
pub mod cr2;
#[doc = "Channel 2 Compare Shadow Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2s](cr2s) module"]
pub type CR2S = crate::Reg<u32, _CR2S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2S;
#[doc = "`read()` method returns [cr2s::R](cr2s::R) reader structure"]
impl crate::Readable for CR2S {}
#[doc = "`write(|w| ..)` method takes [cr2s::W](cr2s::W) writer structure"]
impl crate::Writable for CR2S {}
#[doc = "Channel 2 Compare Shadow Value"]
pub mod cr2s;
#[doc = "Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chc](chc) module"]
pub type CHC = crate::Reg<u32, _CHC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHC;
#[doc = "`read()` method returns [chc::R](chc::R) reader structure"]
impl crate::Readable for CHC {}
#[doc = "`write(|w| ..)` method takes [chc::W](chc::W) writer structure"]
impl crate::Writable for CHC {}
#[doc = "Channel Control"]
pub mod chc;
#[doc = "Dead Time Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtc](dtc) module"]
pub type DTC = crate::Reg<u32, _DTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTC;
#[doc = "`read()` method returns [dtc::R](dtc::R) reader structure"]
impl crate::Readable for DTC {}
#[doc = "`write(|w| ..)` method takes [dtc::W](dtc::W) writer structure"]
impl crate::Writable for DTC {}
#[doc = "Dead Time Control"]
pub mod dtc;
#[doc = "Channel 1 Dead Time Values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc1r](dc1r) module"]
pub type DC1R = crate::Reg<u32, _DC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC1R;
#[doc = "`read()` method returns [dc1r::R](dc1r::R) reader structure"]
impl crate::Readable for DC1R {}
#[doc = "`write(|w| ..)` method takes [dc1r::W](dc1r::W) writer structure"]
impl crate::Writable for DC1R {}
#[doc = "Channel 1 Dead Time Values"]
pub mod dc1r;
#[doc = "Channel 2 Dead Time Values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc2r](dc2r) module"]
pub type DC2R = crate::Reg<u32, _DC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC2R;
#[doc = "`read()` method returns [dc2r::R](dc2r::R) reader structure"]
impl crate::Readable for DC2R {}
#[doc = "`write(|w| ..)` method takes [dc2r::W](dc2r::W) writer structure"]
impl crate::Writable for DC2R {}
#[doc = "Channel 2 Dead Time Values"]
pub mod dc2r;
#[doc = "Timer Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer](timer) module"]
pub type TIMER = crate::Reg<u32, _TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER;
#[doc = "`read()` method returns [timer::R](timer::R) reader structure"]
impl crate::Readable for TIMER {}
#[doc = "`write(|w| ..)` method takes [timer::W](timer::W) writer structure"]
impl crate::Writable for TIMER {}
#[doc = "Timer Value"]
pub mod timer;
#[doc = "Capture Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0v](c0v) module"]
pub type C0V = crate::Reg<u32, _C0V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0V;
#[doc = "`read()` method returns [c0v::R](c0v::R) reader structure"]
impl crate::Readable for C0V {}
#[doc = "Capture Register 0"]
pub mod c0v;
#[doc = "Capture Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1v](c1v) module"]
pub type C1V = crate::Reg<u32, _C1V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1V;
#[doc = "`read()` method returns [c1v::R](c1v::R) reader structure"]
impl crate::Readable for C1V {}
#[doc = "Capture Register 1"]
pub mod c1v;
#[doc = "Capture Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2v](c2v) module"]
pub type C2V = crate::Reg<u32, _C2V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2V;
#[doc = "`read()` method returns [c2v::R](c2v::R) reader structure"]
impl crate::Readable for C2V {}
#[doc = "Capture Register 2"]
pub mod c2v;
#[doc = "Capture Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3v](c3v) module"]
pub type C3V = crate::Reg<u32, _C3V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3V;
#[doc = "`read()` method returns [c3v::R](c3v::R) reader structure"]
impl crate::Readable for C3V {}
#[doc = "Capture Register 3"]
pub mod c3v;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ints](ints) module"]
pub type INTS = crate::Reg<u32, _INTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTS;
#[doc = "`read()` method returns [ints::R](ints::R) reader structure"]
impl crate::Readable for INTS {}
#[doc = "Interrupt Status"]
pub mod ints;
#[doc = "Interrupt Enable Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inte](inte) module"]
pub type INTE = crate::Reg<u32, _INTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTE;
#[doc = "`read()` method returns [inte::R](inte::R) reader structure"]
impl crate::Readable for INTE {}
#[doc = "`write(|w| ..)` method takes [inte::W](inte::W) writer structure"]
impl crate::Writable for INTE {}
#[doc = "Interrupt Enable Control"]
pub mod inte;
#[doc = "Service Request Selector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srs](srs) module"]
pub type SRS = crate::Reg<u32, _SRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRS;
#[doc = "`read()` method returns [srs::R](srs::R) reader structure"]
impl crate::Readable for SRS {}
#[doc = "`write(|w| ..)` method takes [srs::W](srs::W) writer structure"]
impl crate::Writable for SRS {}
#[doc = "Service Request Selector"]
pub mod srs;
#[doc = "Interrupt Status Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sws](sws) module"]
pub type SWS = crate::Reg<u32, _SWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWS;
#[doc = "`write(|w| ..)` method takes [sws::W](sws::W) writer structure"]
impl crate::Writable for SWS {}
#[doc = "Interrupt Status Set"]
pub mod sws;
#[doc = "Interrupt Status Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swr](swr) module"]
pub type SWR = crate::Reg<u32, _SWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWR;
#[doc = "`write(|w| ..)` method takes [swr::W](swr::W) writer structure"]
impl crate::Writable for SWR {}
#[doc = "Interrupt Status Clear"]
pub mod swr;
