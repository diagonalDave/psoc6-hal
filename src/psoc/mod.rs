//! psoc.rs implements high level psoc6 device

use crate::pac::Peripherals;
use crate::gpio::{
    GpioExt,
    Parts,
};

use crate::drivers::{
    system::System,
    cpuss::Cpuss,
    flashc::Flash,
    prot::Prot,
    backup::Backup,
};

pub enum SystemMode{
    Lp,
    Ulp,
    DeepSleep,
    Hibernate,
}
pub enum CpuMode{
    Active,
    Sleep,
    DeepSleep,
}

pub struct Modes{
    cm0p: CpuMode,
    cm4: CpuMode,
    system_mode: SystemMode,
}

pub struct Psoc{
    system: System,
    cpuss: Cpuss,
    flash: Flash,
    pub gpio: Parts,
    prot: Prot,
    modes: Modes,
    backup: Backup,
}
pub mod startup;

impl Psoc{
   
    pub fn new() -> Psoc{
        let p = Peripherals::take().unwrap();
        let gpio =  p.GPIO.split();
        Psoc{
            system: System::from(p.SRSS),
            cpuss: Cpuss::from(p.CPUSS),
            flash: Flash::from(p.FLASHC),
            gpio,
            prot: Prot::from(p.PROT),
            modes: Modes{
                cm0p: CpuMode::Active,
                cm4: CpuMode::Active,
                system_mode: SystemMode::Lp,
            },
            backup: Backup::from(p.BACKUP),
        }
    }
    
}
