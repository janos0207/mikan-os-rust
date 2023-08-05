// #![no_std]
// use bitflags::bitflags;
use core::fmt::Debug;

#[repr(u32)]
#[derive(Debug)]
pub enum OpenProtocolAttributes {
    ByHandleProtocol = 0x01,
    GetProtocol = 0x02,
    TestProtocol = 0x04,
    ByChildController = 0x08,
    ByDriver = 0x10,
    Exclusive = 0x20,
}
