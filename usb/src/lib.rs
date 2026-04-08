#![no_std]

use core::mem::transmute;

struct USBPacketFullSpeed{
    SYNC: u32,
    PID: u8
}

#[derive(Default)]
struct USBSetupPacket{
    bmRequestType: u8,
    bRequest: u8,
    wValue: u16,
    wIndex: u16,
    wLenth: u16
}

#[repr(u8)]
enum USBDirection{ DeviceToHost = 0, HostToDevice = 1 }
#[repr(u8)]
enum PacketType{ Standard = 0, Class = 1, Vendor = 2 }
#[repr(u8)]
enum Recipient{ Device = 0, Interface = 1, Endpoint = 2, Other = 3 }
impl USBSetupPacket{
    pub fn get_direction(&self) -> USBDirection{
        unsafe { transmute(self.bmRequestType & (1 << 7)) }
    }

    pub fn set_direction(&mut self, direction: USBDirection){
        self.bmRequestType &= !(1 << 7);
        self.bmRequestType |= (direction as u8) << 7;
    }

    pub fn get_packet_type(&self) -> PacketType{
        unsafe { transmute(self.bmRequestType & (3 << 5)) }
    }

    pub fn set_packet_type(&mut self, packet_type: PacketType){
        self.bmRequestType &= !(3 << 5);
        self.bmRequestType |= (packet_type as u8) << 5;
    }

    pub fn get_recipient(&self) -> Recipient{
        unsafe { transmute(self.bmRequestType & 0b11111) }
    }

    pub fn set_recipient(&mut self, recipient: Recipient){
        self.bmRequestType &= !0b11111;
        self.bmRequestType |= (recipient as u8) << 5;
    }
}