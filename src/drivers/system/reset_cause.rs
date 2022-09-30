//! reset.rs provides functionality to determine the last reset
//! cause.

use crate::drivers::system::System;

#[derive(PartialEq)]
pub enum ResetCause{
    ActiveFault,
    LostWatchCrystalClock,
    DeepSleepFault,
    MCWDT0Reset,
    MCWDT1Reset,
    MCWDT2Reset,
    MCWDT3Reset,
    SoftwareRequest,
    WDTReset,
    HFClockFrequencyError,
    LostHFClock,
    NoReset,
}

impl System{
    #[inline(always)]
    pub fn last_reset(&self)->ResetCause{
        let mut result = match self.srss.res_cause.read().bits(){
            0x0000_0001 => ResetCause::WDTReset,
            0x0000_0002 => ResetCause::ActiveFault,
            0x0000_0004 => ResetCause::DeepSleepFault,
            0x0000_0008 => ResetCause::LostWatchCrystalClock,
            0x0000_0010 => ResetCause::SoftwareRequest,
            0x0000_0020 => ResetCause::MCWDT0Reset,
            0x0000_0040 => ResetCause::MCWDT1Reset,
            0x0000_0080 => ResetCause::MCWDT2Reset,
            0x0000_0100 => ResetCause::MCWDT3Reset,
            _ => ResetCause::NoReset,
        };
        if result == ResetCause::NoReset {
            result = match self.srss.res_cause2.read().bits(){
                0x0000_0001..=0x0000_ffff => ResetCause::LostHFClock,
                0x0001_0000..=0xffff_0000 => ResetCause:: HFClockFrequencyError,
                _ => ResetCause::NoReset,
            };
        }
        result
    }

}
