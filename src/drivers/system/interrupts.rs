//! interrupt.rs implements functionality to enable cpu interrutps for:
//! - watchdog timer -- ID:WDT
//! - low voltage detector -- ID:HVLVD
//! - clock calibration completion -- ID:CLK_CAL.
//!

use crate::drivers::system::System;

impl System {
    pub fn enable_wdt_intr(&self) -> () {
        self.srss.srss_intr.modify(|_, w| w.wdt_match().set_bit());
    }
    pub fn disable_wdt_intr(&self) -> () {
        self.srss.srss_intr.modify(|_, w| w.wdt_match().clear_bit());
    }
    pub fn enable_clk_cal_intr(&self) -> () {
        self.srss.srss_intr.modify(|_, w| w.clk_cal().set_bit());
    }
    pub fn disable_clk_cal_intr(&self) -> () {
        self.srss.srss_intr.modify(|_, w| w.clk_cal().clear_bit());
    }
    pub fn enable_hvlvd1_intr(&self) -> () {
        self.srss.srss_intr.modify(|_, w| w.hvlvd1().set_bit());
    }
    pub fn disable_hvlvd1_intr(&self) -> () {
        self.srss.srss_intr.modify(|_, w| w.hvlvd1().clear_bit());
    }
}
