//! cpuss implements the cpu subsystems (CPUSS) layer:
//! - Configuration of the Cm0+ and Cm4 cores, interrupts and protection -- cores module
//! - Ram configuration -- ram module
//! - Rom configuration -- rom module
//! - Status, Identity and power control -- utility module
//! - Interrupt configuration -- interrupt module

use crate::pac::CPUSS as CPU_SS;

pub mod cores;
pub mod interrupt;
pub mod utility;

pub struct Cpuss {
    pub cpu_sys: CPU_SS,
}

impl Cpuss {
    fn new(cpuss: CPU_SS) -> Cpuss {
        Cpuss { cpu_sys: cpuss }
    }
}
impl core::convert::From<CPU_SS> for Cpuss {
    fn from(cpuss: CPU_SS) -> Cpuss {
        Cpuss::new(cpuss)
    }
}
