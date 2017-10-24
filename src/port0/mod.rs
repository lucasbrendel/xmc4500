use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 0 Output Register"]
    pub out: OUT,
    #[doc = "0x04 - Port 0 Output Modification Register"]
    pub omr: OMR,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Port 0 Input/Output Control Register 0"]
    pub iocr0: IOCR0,
    #[doc = "0x14 - Port 0 Input/Output Control Register 4"]
    pub iocr4: IOCR4,
    #[doc = "0x18 - Port 0 Input/Output Control Register 8"]
    pub iocr8: IOCR8,
    #[doc = "0x1c - Port 0 Input/Output Control Register 12"]
    pub iocr12: IOCR12,
    _reserved1: [u8; 4usize],
    #[doc = "0x24 - Port 0 Input Register"]
    pub in_: IN,
    _reserved2: [u8; 24usize],
    #[doc = "0x40 - Port 0 Pad Driver Mode 0 Register"]
    pub pdr0: PDR0,
    #[doc = "0x44 - Port 0 Pad Driver Mode 1 Register"]
    pub pdr1: PDR1,
    _reserved3: [u8; 24usize],
    #[doc = "0x60 - Port 0 Pin Function Decision Control Register"]
    pub pdisc: PDISC,
    _reserved4: [u8; 12usize],
    #[doc = "0x70 - Port 0 Pin Power Save Register"]
    pub pps: PPS,
    #[doc = "0x74 - Port 0 Pin Hardware Select Register"]
    pub hwsel: HWSEL,
}
#[doc = "Port 0 Output Register"]
pub struct OUT {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Output Register"]
pub mod out;
#[doc = "Port 0 Output Modification Register"]
pub struct OMR {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Output Modification Register"]
pub mod omr;
#[doc = "Port 0 Input/Output Control Register 0"]
pub struct IOCR0 {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Input/Output Control Register 0"]
pub mod iocr0;
#[doc = "Port 0 Input/Output Control Register 4"]
pub struct IOCR4 {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Input/Output Control Register 4"]
pub mod iocr4;
#[doc = "Port 0 Input/Output Control Register 8"]
pub struct IOCR8 {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Input/Output Control Register 8"]
pub mod iocr8;
#[doc = "Port 0 Input/Output Control Register 12"]
pub struct IOCR12 {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Input/Output Control Register 12"]
pub mod iocr12;
#[doc = "Port 0 Input Register"]
pub struct IN {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Input Register"]
pub mod in_;
#[doc = "Port 0 Pad Driver Mode 0 Register"]
pub struct PDR0 {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Pad Driver Mode 0 Register"]
pub mod pdr0;
#[doc = "Port 0 Pad Driver Mode 1 Register"]
pub struct PDR1 {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Pad Driver Mode 1 Register"]
pub mod pdr1;
#[doc = "Port 0 Pin Function Decision Control Register"]
pub struct PDISC {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Pin Function Decision Control Register"]
pub mod pdisc;
#[doc = "Port 0 Pin Power Save Register"]
pub struct PPS {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Pin Power Save Register"]
pub mod pps;
#[doc = "Port 0 Pin Hardware Select Register"]
pub struct HWSEL {
    register: VolatileCell<u32>,
}
#[doc = "Port 0 Pin Hardware Select Register"]
pub mod hwsel;
