use crate::drivers::system::System;
use crate::pac;
pub struct SystemInfo{
    family_id: u16,
    major_rev: u8,
    minor_rev: u8,
    silicon_id: u16
}

impl System{
    pub fn read_device_id(&self)->DeviceId{
        todo!()
    }
}

