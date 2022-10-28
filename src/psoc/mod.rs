//! psoc.rs implements high level psoc6 device

use crate::gpio::{GpioExt, Parts};
use crate::pac::Peripherals;

use crate::drivers::{
   // backup::Backup,
    //cpuss::Cpuss,
    //flashc::Flash,
    ipc::{
    //    semaphore::Semaphore,
        Channels,
        IntrStructs,
        IpcChannel
    },
    // prot::Prot,
    //system::System,
};


pub mod system_channels;
pub enum SystemMode {
    Lp,
    Ulp,
    DeepSleep,
    Hibernate,
}
pub enum CpuMode {
    Active,
    Sleep,
    DeepSleep,
}

pub struct Modes {
    pub cm0p: CpuMode,
    pub cm4: CpuMode,
    pub system_mode: SystemMode,
}
// pub struct State {
//     pub semaphore: Semaphore,
// }

pub struct Psoc {
    // pub system: System,
    // pub cpuss: Cpuss,
    // pub flash: Flash,
    pub ipc: Channels,
    pub ipc_intr: IntrStructs,
    pub gpio: Parts,
    // pub prot: Prot,
    // pub modes: Modes,
    // pub backup: Backup,
    //pub state: State,
}

impl Psoc {
    pub fn new() -> Psoc {
        let p = Peripherals::take().unwrap();
        let gpio = p.GPIO.split();
        let (ipc, ipc_intr) = p.IPC.split();
        Psoc {
            // system: System::from(p.SRSS),
            // cpuss: Cpuss::from(p.CPUSS),
            // flash: Flash::from(p.FLASHC),
            ipc,
            ipc_intr,
            gpio,
            // prot: Prot::from(p.PROT),
            // modes: Modes {
            //     cm0p: CpuMode::Active,
            //     cm4: CpuMode::Active,
            //     system_mode: SystemMode::Lp,
            // },
            // backup: Backup::from(p.BACKUP),
            // state: State {
            //     semaphore: Semaphore::new(),
            // }
           // ,
        }
    }
}
