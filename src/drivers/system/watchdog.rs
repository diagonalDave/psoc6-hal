//! Initial implementation of the watchodg peripheral for the Cm0p
//! core.
//!The watchdog timer is a 16-bit free running up-counter that
//! works in Active, Sleep, DeepSleep and Hibernate modes.
//!

//use embedded_hal::watchdog::{Watchdog, WatchdogEnable, WatchdogDisable};

use crate::drivers::system::System;
use cortex_m::interrupt;




impl System{
    // /// wdt_unlock is used to wrap any code that requires the watchdog
    // /// to be unlocked to modify. The closure unlocks the watchdog then
    // /// runs the code then relocks the watchdog.The following registers
    // /// require the watchdog to be unlocked for modification.
    // /// - WDT_CTL, WDT_MATCH, WDT_CNT.
    // /// - CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL

    // pub fn wdt_open<F,R>(&self, f: F) -> R
    // where F: FnOnce() -> R,
    // {
    //     self.srss.wdt_ctl.modify(|_,w| w.wdt_lock().clr0());
    //     self.srss.wdt_ctl.modify(|_,w| w.wdt_lock().clr1());

    //     let r = f();

    //     self.srss.wdt_ctl.modify(|_,w| w.wdt_lock().set01());
    //     r
    // }

    //TODO: This should be a closure similar to interrupt::free
    /// wdt_unlock is used to enable modification of:
    /// - WDT_CTL, WDT_MATCH, WDT_CNT.
    /// - CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL
    /// when modifications are complete relock the registers with the
    /// wdt_lock function.
    #[inline(always)]
    pub fn wdt_unlock(&self)-> (){
        self.srss.wdt_ctl.modify(|_,w| w.wdt_lock().clr0());
        self.srss.wdt_ctl.modify(|_,w| w.wdt_lock().clr1());
    }
     /// wdt_lock is used to avoid inadvertant writes to:
    /// - WDT_CTL, WDT_MATCH, WDT_CNT.
    /// - CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL
    /// writes to these registers can only occur after unlocking the
    /// wdt. Use wdt_unlock.
    #[inline(always)]
    pub fn wdt_lock(&self)-> (){
        self.srss.wdt_ctl.modify(|_,w| w.wdt_lock().set01()); 
    }

    #[inline(always)]
    pub fn wdt_is_locked(sys: &System) -> bool{
        sys.srss.wdt_ctl.read().wdt_lock().is_set01()
    }

    
    /// wdt_start starts the watchdog with the timeout specified by timeout_ms.
    /// The max timeout is 6.144 seconds.
    /// Any timeout_ms greater than 6.144seconds will be saturated to
    /// 6.144 seconds. 
    /// The min timeout is 0.001 seconds.
    //`/ A usage example is in examples/watchdog.rs
    pub fn wdt_start(&self, timeout_ms: u32)-> (){
        //unlock the WDT_CTL
         interrupt::free(|_| {
             self.wdt_unlock();
            // wdt_open(|_| {
            //disable the wdt
            self.srss.wdt_ctl.modify(|_,w| w.wdt_en().clear_bit());
            //configure the WDT_MATCH value
            //check the timeout_ms value
            //the timeout value is given by
            // time in secs = ilofreq_period * (2*2^(16-ignore_bits) + wdt_match)
            //or time in secs = (2*2^(16-ignore_bits) + match_bits) / ilofreq
            //match_bits = time_in_secs * ilofreq - 2*2(16-ignore_bits)
            //or match_bits = time_in_msec * ilofreq / 1000.0 - 2*2(16-ignore_bits)
            let match_bits:u16 ;
            let ignore_bits:u8;
            
            if timeout_ms < 2049 {
                ignore_bits = 12;
                match_bits = (timeout_ms as f32  * 32.0 - 32.0) as u16; //32 = 2*2(16-12) (- (* 2049 32) 32)65536
            }else if timeout_ms < 4096 {
                ignore_bits = 1;
                match_bits = (timeout_ms as f32 * 32.0 - 65536.0) as u16; //65536 = 2*2(16-1) (- (* 4096 32) 65536)65536
            }else if timeout_ms < 6144{
                ignore_bits = 0;
                match_bits = (timeout_ms as f32 * 32.0 - 131072.0) as u16; //131072 = 2*2(16) (- (* 6144.0 32) 131072)65536.0
            }else{
                ignore_bits = 0;
                match_bits = 0xffff;
            }

            self.srss.wdt_match.modify(|_, w| unsafe{w.bits(match_bits as u32)});
            self.srss.wdt_match.modify(|_,w| unsafe{w.ignore_bits().bits(ignore_bits)});
            //enable iloclock (should be enabled by default.
            self.srss.clk_ilo_config.modify(|_,w| w.enable().set_bit());
            //enable the wdt
            self.srss.wdt_ctl.modify(|_,w| w.wdt_en().set_bit());
            //clear any interrupts
            self.srss.srss_intr.modify(|_,w| w.wdt_match().set_bit());
            // //write the interrupt mask to forward interrupts to cpu.
            // self.srss.wdt_intr_mask.modify(|_, w| w.wdt_match().set_bit());
                //lock the wdt.
             });           
           self.wdt_lock();
//        });
    }
    /// wdt_feed is used to provide a periodic response to the watchdog
    /// to ensure the mcu is not reset.
    #[inline(always)]
    pub fn wdt_feed(&self) -> (){
        interrupt::free(|_| {
            //set the srss_intr wdt_match bit to clear interrupt.
            self.srss.srss_intr.modify(|_,w| w.wdt_match().set_bit());
        });
    }
    /// wdt_disable stops wdt operation.
    #[inline(always)]
    pub fn wdt_disable(&self)-> (){
        interrupt::free(|_| {
            self.wdt_unlock();
            self.srss.wdt_ctl.modify(|_,w| w.wdt_en().clear_bit());
            self.wdt_lock();
        });
    }
}
