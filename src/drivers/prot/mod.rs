//! The prot library implements functions to manage the Protection
//! Units that enforce security based on operations. A protection unit
//! allows or restricts bus transfers based on properties of the
//! transfer. The rules that determine protection are implemented in
//! protection structures (a register structure). A protection structure
//! defines the protected address space and the protection
//! attributes. The hardware that evaluates these protection structures,
//! to restrict or permit access, is the protection unit. The PSoC
//! device has different types of protection units such as MPU, SMPU,
//! and PPU. Each have a distinct set of protection structures, which
//! helps define different protection regions and their attributes.
//! See trm Chapter 9 pp70

use crate::pac::PROT;

/// ProtBusMaster identify all the bus masters using the
/// shared memory protection unit (SMPU)
pub enum ProtBusMaster {
    IdCm0,
    IdCrypto,
    IdDw0,
    IdDw1,
    IdCm4,
    IdTc,
}
/// ProtContext restricts the protection contexts to the
/// available range.
pub enum ProtContext {
    NoContext,
    Context1,
    Context2,
    Context3,
    Context4,
    Context5,
    Context6,
    Context7,
    Context8,
    Context9,
    Context10,
    Context11,
    Context12,
    Context13,
    Context14,
    Context15,
}
pub struct Prot {
    pub prot: PROT,
}

pub enum Error {
    UnknownContext,
}

impl Prot {
    fn new(prot: PROT) -> Self {
        Self { prot }
    }
    #[inline]
    pub fn from(prot: PROT) -> Self {
        Self::new(prot)
    }
    #[inline]
    pub fn read_active_context(&self, master: ProtBusMaster) -> ProtContext {
        //TODO::Not sure how all this works so will leave for startup.
        let _ = master;
        // //let index = Prot::bus_master_to_index(master);
        // let mpux = match master{
        //     IdCm0 => self.prot.mpu0,
        //     IdCrypto => self.prot.mpu1,
        //     IdDw0 => self.prot.mpu2,
        //     IdDw1 => self.prot.mpu3,
        //     IdCm4 => self.prot.mpu14,
        //     IdTc => self.prot.mpu15,
        // };

        // match mpux.ms_ctl.read().pc().bits(){
        //     0x00 => Ok(ProtContext::NoContext),
        //     0x01 => Ok(ProtContext::Context1),
        //     0x02 => Ok(ProtContext::Context2),
        //     0x03 => Ok(ProtContext::Context3),
        //     0x04 => Ok(ProtContext::Context4),
        //     0x05 => Ok(ProtContext::Context5),
        //     0x06  => Ok(ProtContext::Context6),
        //     0x07 => Ok(ProtContext::Context7),
        //     0x08 => Ok(ProtContext::Context8),
        //     0x09 => Ok(ProtContext::Context9),
        //     0x10 => Ok(ProtContext::Context10),
        //     0x11 => Ok(ProtContext::Context11),
        //     0x12 => Ok(ProtContext::Context12),
        //     0x13 => Ok(ProtContext::Context13),
        //     0x14 => Ok(ProtContext::Context14),
        //     0x15 => Ok(ProtContext::Context15),
        //     _ => Err(Error::UnknownContext),
        // }
        ProtContext::NoContext
    }
    #[allow(dead_code)]
    #[inline]
    fn bus_master_to_index(bus_master: ProtBusMaster) -> u8 {
        match bus_master {
            ProtBusMaster::IdCm0 => 0,
            ProtBusMaster::IdCrypto => 1,
            ProtBusMaster::IdDw0 => 2,
            ProtBusMaster::IdDw1 => 3,
            ProtBusMaster::IdCm4 => 14,
            ProtBusMaster::IdTc => 15,
        }
    }
}
