//! clocks.rs is a driver that provides and interface to the clocking
//! functionality for PSOC6. Use of this module requires some knowledge
//! of underlying clock registers, sequencing and dependant actions.
//! See trm Chapter 20  pp220.
//! Preferably use the HAL clocks.rs
//! The PSoC 6 MCU clock system includes these resources:
//! - Three internal clock sources:
//!   - 8-MHz internal main oscillator (IMO)
//!   - 32-kHz internal low-speed oscillator (ILO)
//!   - Precision 32-kHz internal low-speed oscillator (PILO)
//! - Three external clock sources
//!   - External clock (EXTCLK) generated using a signal from an I/O
//!     pin
//!   - External 16–35 MHz crystal oscillator (ECO)
//!   - External 32-kHz watch crystal oscillator (WCO)
//! - One frequency lock loop (FLL) with 24–100 MHz output range
//! - One phase-locked loop (PLL) with 10.625–150 MHz output range
//!
//! The clocks are used in the device as clock trees with the device
//! having the following clock trees:
//! - Path clocks:
//!   - CLK_PATH0--contains the frequency locked loop (FLL). The FLL
//!                can be bypassed.
//!   - CLK_PATH1--contains the phase locked loop (PLL). The PLL can
//!                be bypassed.
//!   - CLK_PATH2..4--are direct connections to the high frequency root
//!                clocks
//! - High Frequency Root Clocks (CLK_HF\[i\]):
//!   - CLK_HF\[0\]--root clock for CPUs, PERI and AHB infrastructure.
//!   - CLK_HF\[1\]--root clock for the PDM/PCM and I2S audio subsystem.
//!   - CLK_HF\[2\]--root clock for the Serial Memory Interface
//!                subsystem.
//!   - CLK_HF\[3\]--root clock for USB communication.
//!   - CLK_HF\[4\]--clock output on clk_ext pin when configures as an
//!                output.
//!

#![deny(unsafe_code)]
#![deny(warnings)]

use cortex_m::asm::delay;

use crate::drivers::system::System;
use core::result::Result;

pub enum Clocks {
    Imo,    // 8-MHz internal main oscillator
    Ilo,    // 32-kHz internal low-speed oscillator
    Pilo,   // Precision 32.768-kHz internal low-speed oscillator
    ExtClk, // External clock
    Eco,    // External 16–35 MHz crystal oscillator
    Wco,    // External 32.768-kHz watch crystal oscillator
    Fll,    // Frequency locked loop 24-100Mhz
    Pll,    // Phase locked loop 10.625-150Mhz
}

pub enum RootClocks {
    CpuPeriAhb = 0, // Root clock for the CPUs, PERI, and AHB
    PdmPcmI2s = 1,  // Root clock for the PDM/PCM and I2S audio subsystem
    Smif = 2,       // Root clock for the Serial memory interface.
    Usb = 3,        // Root clock for USB communication.
    ClkOut = 4,     // Root clock for the clk_ext_pin output.
}

pub enum Divider {
    NoDiv,
    Div2,
    Div4,
    Div8,
}

pub enum PathSource {
    Imo,
    ExtClk,
    Eco,
    AltHf,
    DsiMux,
}
pub enum ClockPath {
    Path0 = 0,
    Path1 = 1,
    Path2 = 2,
    Path3 = 3,
    Path4 = 4,
}
/// ClocksPathNumber represents the Path select index.
/// the enum is constrained to the available values.
pub enum SelectChannelNumber {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}
pub enum DsiMux {
    DsiOut0 = 0,
    DsiOut1 = 1,
    Ilo = 16,
    Wco = 17,
    Pilo = 19,
}
#[derive(PartialEq)]
pub enum Error {
    NoError,
    UnknownLfClkSource,
    UnknownPathSource,
    FllCouldNotBeConfigured,
    FllCouldNotBeStarted,
    FllStartupCouldNotBeCompletedBeforeTimeout,
    FllStartupFailedCcoNotReady,
    FllStartupFailedFllCouldNotBeLocked,
}
/// FLL configuration enums
pub struct FllConfig {
    fll_mult: u32,
    ref_div: u16,
    cco_range: FllCco,
    output_div_enable: FllOutputDiv,
    lock_tolerance: u16, // 9 bits are used to configure the error value from 0-511 bits.
    igain: FllGain,      // 4 bits wide value for igain configurations
    pgain: FllGain,
    settling_count: u16, // 0-8191
    cco_freq: u16,
}

impl FllConfig {
    pub fn default() -> Self {
        Self {
            fll_mult: 500,
            ref_div: 20,
            cco_range: FllCco::Range4, //cco base frequency range
            output_div_enable: FllOutputDiv::Enable,
            lock_tolerance: 10,
            igain: FllGain::Mult2,
            pgain: FllGain::Div8,
            settling_count: 8,
            cco_freq: 355,
        }
    }
}
pub enum FllOutputDiv {
    Disable,
    Enable,
}
pub enum FllGain {
    Div256 = 0,
    Div128 = 1,
    Div64 = 2,
    Div32 = 3,
    Div16 = 4,
    Div8 = 5,
    Div4 = 6,
    Div2 = 7,
    Div1 = 8,
    Mult2 = 9,
    Mult4 = 10,
    Mult8 = 11,
}
/// ClocksFllCco represents the  target frequency range
/// of the Fll configuration.
/// Range0--target frequency range 48-64Mhz  
/// Range1--target frequency range 65-85Mhz  
/// Range2--target frequency range 85-113Mhz
/// Range3--target frequency range 113-150Mhz
/// Range4--target frequency range 150-200Mhz
pub enum FllCco {
    Range0,
    Range1,
    Range2,
    Range3,
    Range4,
}

/// Clock configuration
/// The clocks are organised as clock trees. The configuration of the:
/// - Low Frequency Clock
/// - Timer Clock
/// - Analog Pump Clock
/// clock trees are a call to a single method.
/// Configuration of the High Frequency Root Clocks (HF_CLK) requires:
/// 1. Selection of the root clock source for the HF_CLK.
/// 2. Set the root clock divider
/// 3. Enable the root clock.
/// 4. Configuration of a Path Clock Sources.
/// 5. Configuration of the FLL (if used).
/// Note: When clocks have been modified the wait states must be
/// checked to ensure they are within tolerance for the new clock
/// frequency.

impl System {
    ///
    #[allow(dead_code)]
    pub(crate) fn configure_system_clocks(&self) -> () {
        let _ = self.configure_lfclk_source(Clocks::Ilo);

        self.configure_root_clk_source(RootClocks::CpuPeriAhb, ClockPath::Path0);
        //Set  8Mhz IMO for al clock paths.
        self.configure_path_source(ClockPath::Path0, PathSource::Imo);
        self.configure_path_source(ClockPath::Path1, PathSource::Imo);
        self.configure_path_source(ClockPath::Path2, PathSource::Imo);
        self.configure_path_source(ClockPath::Path3, PathSource::Imo);
        self.configure_path_source(ClockPath::Path4, PathSource::Imo);

        //configure the fll
        let fll_config = FllConfig::default();
        self.configure_fll(fll_config);
        match self.start_fll(200_000) {
            Ok(()) => {}
            Err(e) => {
                let _ = e;
                loop {}
            } // nothing to do but fail if fll can't start.
        }

        self.configure_timer_clock();
        self.configure_clock_pump();
        self.configure_ilo_hibernate();
    }

    #[inline]
    pub fn start_system_clocks(&self) -> () {
        self.start_lfclk();
        self.start_root_clk(RootClocks::CpuPeriAhb);
        self.start_timer_clock();
        self.start_clock_pump();
    }
    /// select_lfclk_source selects the clock source for the low frequency clock tree.
    /// It takes a clocks paramter that can be one of:
    ///  - Ilo -- relatively low accuracy and low power.
    ///  - Wco -- high accuracy clock source primary clock for RTC.
    ///  - Pilo -- works in deep sleep and active modes, higher accuracy clock e.g ECO.
    /// It returns an error if a non-specified clock source is passed in.
    /// Note the watchdog must be unlocked for configuration changes to
    /// clk_select to set.
    ///
    #[inline(always)]
    pub fn configure_lfclk_source(&self, clk: Clocks) -> Result<(), Error> {
        self.wdt_unlock();
        match clk {
            Clocks::Ilo => {
                self.srss.clk_select.modify(|_, w| w.lfclk_sel().ilo());
                self.wdt_lock();
                Ok(())
            }
            Clocks::Wco => {
                self.srss.clk_select.modify(|_, w| w.lfclk_sel().wco());
                self.wdt_lock();
                Ok(())
            }
            Clocks::Pilo => {
                self.srss.clk_select.modify(|_, w| w.lfclk_sel().pilo());
                self.wdt_lock();
                Ok(())
            }
            _ => {
                self.wdt_lock();
                Err(Error::UnknownLfClkSource)
            }
        }
    }

    #[inline(always)]
    pub fn start_lfclk(&self) -> () {
        self.wdt_unlock();
        self.srss.clk_ilo_config.modify(|_, w| w.enable().set_bit());
        self.wdt_lock();
    }
    /// stop_lfclk provides the Watchdog (WDT) clk and can be used
    /// to clock the backup domain.
    /// The LF_CLK is enabled by default and care should be
    /// used when disabling the clock
    #[inline(always)]
    pub fn stop_lfclk(&self) -> () {
        self.wdt_unlock();
        self.srss
            .clk_ilo_config
            .modify(|_, w| w.enable().clear_bit());
        self.wdt_lock();
    }

    /// clocks_init_path_source configures a source clock for the path.
    /// Input paramters:
    ///  - path_num: a ClocksSelectChannelNumber member.
    ///  - path_source: a ClocksPathSource member.
    /// Selecting a DsiMux path source requires configuration of the dsi_mux. See clocks_init_dsi_mux_source.
    ///
    #[inline(always)]
    pub fn configure_path_source(&self, path_num: ClockPath, path_source: PathSource) -> () {
        match path_source {
            PathSource::Imo => {
                self.srss.clk_path_select[path_num as usize].modify(|_, w| w.path_mux().imo())
            }
            PathSource::ExtClk => {
                self.srss.clk_path_select[path_num as usize].modify(|_, w| w.path_mux().extclk())
            }
            PathSource::Eco => {
                self.srss.clk_path_select[path_num as usize].modify(|_, w| w.path_mux().eco())
            }
            PathSource::AltHf => {
                self.srss.clk_path_select[path_num as usize].modify(|_, w| w.path_mux().althf())
            }
            PathSource::DsiMux => {
                self.srss.clk_path_select[path_num as usize].modify(|_, w| w.path_mux().dsi_mux())
            }
        }
    }
    /// clocks_init_dsi_mux_source is used to select a low frequency clock source when
    /// the DsiMux option is selected for a path clock.
    #[inline(always)]
    pub fn configure_dsi_mux_source(
        &self,
        dsi_select: SelectChannelNumber,
        dsi_mux_select: DsiMux,
    ) -> () {
        match dsi_mux_select {
            DsiMux::DsiOut0 => {
                self.srss.clk_dsi_select[dsi_select as usize].modify(|_, w| w.dsi_mux().dsi_out0())
            }
            DsiMux::DsiOut1 => {
                self.srss.clk_dsi_select[dsi_select as usize].modify(|_, w| w.dsi_mux().dsi_out1())
            }
            DsiMux::Ilo => {
                self.srss.clk_dsi_select[dsi_select as usize].modify(|_, w| w.dsi_mux().ilo())
            }
            DsiMux::Wco => {
                self.srss.clk_dsi_select[dsi_select as usize].modify(|_, w| w.dsi_mux().wco())
            }
            DsiMux::Pilo => {
                self.srss.clk_dsi_select[dsi_select as usize].modify(|_, w| w.dsi_mux().pilo())
            }
        }
    }

    /// select_hf_root_clk_source selects the clock source for the high
    /// frequency clock.
    /// hf_index is the root clock to be configured.
    /// clk is the clk source for the root clock.
    #[inline(always)]
    pub fn configure_root_clk_source(&self, hf_index: RootClocks, clk_path: ClockPath) -> () {
        match clk_path {
            ClockPath::Path0 => {
                self.srss.clk_root_select[hf_index as usize].modify(|_, w| w.root_mux().path0());
            }
            ClockPath::Path1 => {
                self.srss.clk_root_select[hf_index as usize].modify(|_, w| w.root_mux().path1());
            }
            ClockPath::Path2 => {
                self.srss.clk_root_select[hf_index as usize].modify(|_, w| w.root_mux().path2());
            }
            ClockPath::Path3 => {
                self.srss.clk_root_select[hf_index as usize].modify(|_, w| w.root_mux().path3());
            }
            ClockPath::Path4 => {
                self.srss.clk_root_select[hf_index as usize].modify(|_, w| w.root_mux().path4());
            }
        }
    }
    #[inline(always)]
    pub fn configure_root_clk_div(&self, hf_index: RootClocks, clk_div: Divider) -> () {
        match clk_div {
            Divider::NoDiv => {
                self.srss.clk_root_select[hf_index as usize].modify(|_, w| w.root_div().no_div());
            }
            Divider::Div2 => {
                self.srss.clk_root_select[hf_index as usize].modify(|_, w| w.root_div().div_by_2());
            }
            Divider::Div4 => {
                self.srss.clk_root_select[hf_index as usize].modify(|_, w| w.root_div().div_by_4());
            }
            Divider::Div8 => {
                self.srss.clk_root_select[hf_index as usize].modify(|_, w| w.root_div().div_by_8());
            }
        }
    }
    #[inline(always)]
    pub fn start_root_clk(&self, hf_index: RootClocks) -> () {
        self.srss.clk_root_select[hf_index as usize].modify(|_, w| w.enable().set_bit());
    }

    #[allow(unsafe_code)]
    pub fn configure_fll(&self, fll_config: FllConfig) -> () {
        //see pp228 20.4.2.2 Enabling and Disabling the FLL.
        //Check the Fll is disabled.
        self.stop_fll();
        //Configuration starts
        self.srss
            .clk_fll_config
            .modify(|_, w| unsafe { w.fll_mult().bits(fll_config.fll_mult) });
        self.srss
            .clk_fll_config2
            .modify(|_, w| unsafe { w.fll_ref_div().bits(fll_config.ref_div) });
        match fll_config.cco_range {
            FllCco::Range0 => self
                .srss
                .clk_fll_config4
                .modify(|_, w| w.cco_range().range0()),
            FllCco::Range1 => self
                .srss
                .clk_fll_config4
                .modify(|_, w| w.cco_range().range1()),
            FllCco::Range2 => self
                .srss
                .clk_fll_config4
                .modify(|_, w| w.cco_range().range2()),
            FllCco::Range3 => self
                .srss
                .clk_fll_config4
                .modify(|_, w| w.cco_range().range3()),
            FllCco::Range4 => self
                .srss
                .clk_fll_config4
                .modify(|_, w| w.cco_range().range4()),
        }
        match fll_config.output_div_enable {
            FllOutputDiv::Enable => self
                .srss
                .clk_fll_config
                .modify(|_, w| w.fll_output_div().set_bit()),
            FllOutputDiv::Disable => self
                .srss
                .clk_fll_config
                .modify(|_, w| w.fll_output_div().clear_bit()),
        }
        self.srss
            .clk_fll_config2
            .modify(|_, w| unsafe { w.lock_tol().bits(fll_config.lock_tolerance) });
        let igain = fll_config.igain as u8;
        self.srss
            .clk_fll_config3
            .modify(|_, w| unsafe { w.fll_lf_igain().bits(igain) });
        let pgain = fll_config.pgain as u8;
        self.srss
            .clk_fll_config3
            .modify(|_, w| unsafe { w.fll_lf_pgain().bits(pgain) });
        let settling_count = fll_config.settling_count as u16;
        self.srss
            .clk_fll_config3
            .modify(|_, w| unsafe { w.settling_count().bits(settling_count) });
        let cco_freq = fll_config.cco_freq as u16;
        self.srss
            .clk_fll_config4
            .modify(|_, w| unsafe { w.cco_freq().bits(cco_freq) });
    }

    pub fn start_fll(&self, mut timeout_us: u32) -> Result<(), Error> {
        if self.srss.clk_fll_config.read().fll_enable().bit_is_clear() {
            let mut error = match timeout_us {
                0 => Error::FllStartupCouldNotBeCompletedBeforeTimeout,
                _ => Error::NoError,
            };
            //Config done need to start up the fll.
            self.srss
                .clk_trim_cco_ctl
                .modify(|_, w| w.enable_cnt().set_bit()); //start the cco stabilisation counter.
                                                          //To enable the FLL, first enable the CCO by writing CLK_FLL_CONFIG4.CCO_ENABLE=1 and
            self.srss
                .clk_fll_config4
                .modify(|_, w| w.cco_enable().set_bit());
            //wait until CLK_FLL_STATUS.CCO_READY==1
            while !self.srss.clk_fll_status.read().cco_ready().bit_is_set()
                && error == Error::NoError
            {
                match timeout_us > 0 {
                    true => {
                        timeout_us -= 1;
                        delay(100);
                    }
                    false => error = Error::FllStartupFailedCcoNotReady,
                }
            }
            //Next, ensure the reference clock has stabilized and CLK_FLL_CONFIG3.BYPASS_SEL=FLL_REF.
            self.srss
                .clk_fll_config3
                .modify(|_, w| w.bypass_sel().fll_ref());
            self.srss.clk_fll_config3.read().bypass_sel().is_fll_ref(); // read ensures write has completed.
                                                                        //  Next, write FLL_ENABLE=1 and wait until
            if timeout_us > 0 {
                self.srss
                    .clk_fll_config
                    .modify(|_, w| w.fll_enable().set_bit());
            }
            // CLK_FLL_STATUS.LOCKED==1.
            //wait for the system to lock

            while self.srss.clk_fll_status.read().locked().bit_is_clear() && error == Error::NoError
            {
                match timeout_us > 0 {
                    true => {
                        timeout_us -= 1;
                        delay(100);
                    }
                    false => error = Error::FllStartupFailedFllCouldNotBeLocked,
                }
            }

            // Finally, write CLK_FLL_CONFIG3.BYPASS_SEL=FLL_OUT
            // to switch to the FLL output. It takes seven reference clock cycles plus four FLL output cycles to
            // switch to the FLL output. Do not disable the FLL before this time completes.

            self.srss
                .clk_fll_config3
                .modify(|_, w| w.bypass_sel().fll_out());

            cortex_m::asm::delay(100);
            if timeout_us <= 0 {
                Err(error)
            } else {
                Ok(())
            }
        } else {
            //Fll already started
            Ok(())
        }
    }
    #[inline(always)]
    pub fn stop_fll(&self) -> () {
        if self.srss.clk_fll_config.read().fll_enable().bit_is_set() {
            //When it is enabled disable.
            self.srss
                .clk_fll_config3
                .modify(|_, w| w.bypass_sel().fll_ref());
            self.srss.clk_fll_config3.read().bypass_sel().is_fll_ref();

            self.srss
                .clk_fll_config
                .modify(|_, w| w.fll_enable().clear_bit());
            delay(1000);
            self.srss
                .clk_fll_config4
                .modify(|_, w| w.cco_enable().clear_bit());
        } else {
            //job is done.
        }
    }
    /// configure_timer_clock is a minimal implementation to
    /// enable start up.
    /// TODO: needs full implementation.
    #[allow(unsafe_code)]
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn configure_timer_clock(&self) -> () {
        self.srss.clk_timer_ctl.modify(|_, w| w.timer_sel().imo()); //choose the imo as the source
        self.srss
            .clk_timer_ctl
            .modify(|_, w| unsafe { w.timer_div().bits(0x0) }); // set the divider to 1
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn start_timer_clock(&self) -> () {
        self.srss.clk_timer_ctl.modify(|_, w| w.enable().set_bit());
    }
    /// configure_clock_pump is a minimal implementation to enable
    /// start up.
    /// TODO: needs full implementation.
    #[allow(dead_code)]
    #[allow(unsafe_code)]
    #[inline(always)]
    pub(crate) fn configure_clock_pump(&self) -> () {
        self.srss
            .clk_select
            .modify(|_, w| unsafe { w.pump_sel().bits(0x00) });
        self.srss.clk_select.modify(|_, w| w.pump_div().div_by_4());
    }
    #[inline(always)]
    pub(crate) fn start_clock_pump(&self) -> () {
        self.srss
            .clk_select
            .modify(|_, w| w.pump_enable().set_bit());
    }

    /// configure_clock_pump is a minimal implementation to enable
    /// start up.
    /// TODO: needs full implementation.
    #[allow(dead_code)]
    #[allow(unsafe_code)]
    #[inline(always)]
    pub(crate) fn configure_ilo_hibernate(&self) -> () {
        self.wdt_unlock();
        self.srss
            .clk_ilo_config
            .modify(|_, w| w.ilo_backup().set_bit());
        self.wdt_lock();
    }
} // impl System
