//! power_sam.rs implements functions to control the core LDO.
use crate::drivers::system::System;
use crate::psoc::SystemMode;
use cortex_m::{asm::delay, interrupt::free};

/// impl System for power_sam module.
impl System {
    /// config_core_ldo takes a SystemMode parameter, only Ulp and Lp mode have any effect.
    /// then sets the LDO to match.
    /// Prior to calling this method the lf_clk, hf_clk and wait states should
    /// be configured within the System mode ranges for each mode respectively.
    ///```
    /// psoc.system.configure_hf_clk(33_000_000);
    /// psoc.system.configure_wait_states(33_000_000, SystemMode::Ulp);
    /// psoc.flash.configure_wait_states(33_000_000, SystemMode::Ulp);
    /// psoc.system.config_core_ldo(SystemMode::Ulp);
    /// psoc.flash.configure_ldo_mode(SystemMode::Ulp);
    /// //...application code in Ulp mode.
    ///```
    /// Future work will simplify this call to a single
    pub fn configure_ldo_mode(&self, mode: &SystemMode) -> () {
        free(|_| {
            match mode {
                SystemMode::Ulp => {
                    //TODO CLK_HF suitable for ULP i.e. < 33Mhz
                    self.srss
                        .pwr_trim_pwrsys_ctl
                        .modify(|_, w| unsafe { w.act_reg_trim().bits(0x07) });
                    delay(900);
                    // //update read write margin for RAM and ROM in LP mode
                    // self.srss.config_wait_state(SystemMode::Ulp);
                }
                SystemMode::Lp => {
                    /*Minimum to config 1V1 mode.*/
                    //setup delay to stabilise voltage.
                    //to get a 9us delay using cortex_m at 100MHz clock
                    self.srss
                        .pwr_trim_pwrsys_ctl
                        .modify(|_, w| unsafe { w.act_reg_trim().bits(0x17) });
                    delay(900);
                    // //update read write margin for RAM and ROM in LP mode
                    // self.srss.config_wait_state(SystemMode::Lp);
                }
                _ => {}
            }
        });
    }
}
