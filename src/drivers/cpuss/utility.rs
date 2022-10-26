//! utility.rs

use crate::drivers::cpuss::Cpuss;
use crate::psoc::SystemMode;

impl Cpuss {
    /// wait states implemented as TRM pp 33 sect 4.2.1.1
    #[inline(always)]
    pub fn configure_wait_states(&self, hf_clock_freq: u32, sys_mode: &SystemMode) -> () {
        let ws_slow: u8;
        let ws_fast = 0x00;
        if hf_clock_freq > 100_000_000 {
            match sys_mode {
                SystemMode::Ulp => ws_slow = 0x01,
                _ => ws_slow = 0x01,
            }
        } else if hf_clock_freq > 25_000_000 {
            match sys_mode {
                SystemMode::Ulp => ws_slow = 0x01,
                _ => ws_slow = 0x00,
            }
        } else {
            ws_slow = 0x00;
        }
        self.cpu_sys
            .rom_ctl
            .modify(|_, w| unsafe { w.slow_ws().bits(ws_slow) });
        self.cpu_sys
            .rom_ctl
            .modify(|_, w| unsafe { w.fast_ws().bits(ws_fast) });

        // SRAM
        self.cpu_sys
            .ram0_ctl0
            .modify(|_, w| unsafe { w.slow_ws().bits(ws_slow) });
        self.cpu_sys
            .ram0_ctl0
            .modify(|_, w| unsafe { w.fast_ws().bits(ws_fast) });
        //ramc1 and ramc2 mentioned in code but not in trm.
    }
    /// configure_systick_source is a minimal implementation to enable
    /// start up.
    /// TODO: needs full implementation.
    #[allow(unsafe_code)]
    #[inline(always)]
    pub fn configure_systick_source(&self) -> () {
        self.cpu_sys
            .systick_ctl
            .modify(|_, w| unsafe { w.clock_source().bits(0x00) }); // Sets the ilo as source.
    }
}
