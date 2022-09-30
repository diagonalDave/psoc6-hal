//! system.rs implements a driver for the system resources (SRSS) registers
//! essentially all the system subsystems:
//! - Power supply and Monitoring -- power_sam
//! - Power mode -- power_mode
//! - Backup system -- back_sys
//! - Clocks -- clocks
//! - Reset Cause -- reset_cause
//! - I/O system -- io_sys
//! - Watchdog -- watchdog
//! - Trigger Multiplexer -- trigger_mux
//! - Profiler -- profiler

pub use reset_cause::ResetCause;
pub use clocks::{Clocks,
                 RootClocks,
                 Divider,
                 PathSource,
                 ClockPath,
                 SelectChannelNumber,
                 DsiMux,
                 Error,
                 FllConfig,
                 FllOutputDiv,
                 FllGain,
                 FllCco,
};

use crate::pac::SRSS;

pub mod watchdog;
pub mod reset_cause;
pub mod clocks;
pub mod power_sam;
pub mod interrupts;

pub struct System{
    pub srss: SRSS,
}

impl System{
    fn new(srss: SRSS) -> System{
        System{
            srss,
        }
    }
    #[allow(dead_code)]
    fn free(self) -> SRSS{
        self.srss
    }
}

impl core::convert::From<SRSS> for System{
    fn from(srss: SRSS) -> System{
        System::new(srss)
    }
}

