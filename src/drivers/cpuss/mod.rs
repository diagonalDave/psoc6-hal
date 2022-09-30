//! cpuss implements the cpu subsystems (CPUSS) layer:
//! - Configuration of the Cm0+ and Cm4 cores, interrupts and protection -- cores module 
//! - Ram configuration -- ram module
//! - Rom configuration -- rom module
//! - Status, Identity and power control -- utility module

use crate::pac::CPUSS;


pub mod cores;
pub mod utility;


pub struct Cpuss{
    pub cpuss: CPUSS,
}

impl Cpuss{
    fn new(cpuss: CPUSS) -> Cpuss{
        Cpuss{
            cpuss,
        }
    }
}
impl core::convert:: From<CPUSS> for Cpuss{
    fn from(cpuss:CPUSS) -> Cpuss{
        Cpuss::new(cpuss)
    }
}
