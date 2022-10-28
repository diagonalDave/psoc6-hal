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

pub use clocks::{
    ClockPath, Clocks, Divider, DsiMux, FllCco, FllConfig, FllGain, FllOutputDiv,
    PathSource, RootClocks, SelectChannelNumber,
};
pub use reset_cause::ResetCause;

use crate::pac::SRSS;

pub mod clocks;
pub mod interrupts;
pub mod power_sam;
pub mod reset_cause;
pub mod watchdog;

pub struct System {
    pub srss: SRSS,
}

impl System {
    fn new(srss: SRSS) -> System {
        System { srss }
    }
    #[allow(dead_code)]
    fn free(self) -> SRSS {
        self.srss
    }
}

impl core::convert::From<SRSS> for System {
    fn from(srss: SRSS) -> System {
        System::new(srss)
    }
}
