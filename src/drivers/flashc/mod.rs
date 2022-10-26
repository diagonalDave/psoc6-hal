//! cpuss implements the cpu subsystems (CPUSS) layer:
//! - Configuration of the Cm0+ and Cm4 cores, interrupts and protection -- cores module
//! - Ram configuration -- ram module
//! - Rom configuration -- rom module
//! - Status, Identity and power control -- utility module

use crate::pac::FLASHC;
use crate::psoc::SystemMode;

pub struct Flash {
    pub flash: FLASHC,
}

impl Flash {
    fn new(flash: FLASHC) -> Self {
        Self { flash }
    }
    #[inline(always)]
    pub fn configure_wait_states(&self, hf_clock_freq: u32, sys_mode: &SystemMode) -> () {
        let ws_main: u8;
        match sys_mode {
            SystemMode::Ulp => {
                if hf_clock_freq > 33_000_000 {
                    ws_main = 0x02;
                } else if hf_clock_freq > 16_000_000 {
                    ws_main = 0x01;
                } else {
                    ws_main = 0x00;
                }
            }
            _ => {
                if hf_clock_freq > 120_000_000 {
                    ws_main = 0x04;
                } else if hf_clock_freq > 87_000_000 {
                    ws_main = 0x03;
                } else if hf_clock_freq > 58_000_000 {
                    ws_main = 0x02
                } else if hf_clock_freq > 29_000_000 {
                    ws_main = 0x01;
                } else {
                    ws_main = 0x00;
                }
            }
        }
        self.flash
            .flash_ctl
            .modify(|_, w| unsafe { w.main_ws().bits(ws_main) });
    }

    #[inline(always)]
    pub fn configure_ldo_mode(&self, sys_mode: &SystemMode) -> () {
        match sys_mode {
            SystemMode::Ulp => {
                //Set voltage for flash. vcc_sel 0 for LP mode 1 for ULP mode
                self.flash
                    .fm_ctl
                    .ana_ctl0
                    .modify(|_, w| w.vcc_sel().set_bit())
            }
            SystemMode::Lp => {
                //Set voltage for flash. vcc_sel 0 for LP mode 1 for ULP mode
                self.flash
                    .fm_ctl
                    .ana_ctl0
                    .modify(|_, w| w.vcc_sel().clear_bit());
            }
            _ => {}
        }
    }
}

impl core::convert::From<FLASHC> for Flash {
    fn from(flash: FLASHC) -> Flash {
        Flash::new(flash)
    }
}
