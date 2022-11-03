use crate::drivers::cpuss::Cpuss;
use crate::pac::cpuss::cm4_pwr_ctl::PWR_MODE_A;
use cortex_m::interrupt::free;
// pub enum CoreStatus{
//     CM4Enabled,      //< The Cortex-M4 core is enabled: power on, clock on, no isolate, no reset and no retain. 
//     CM4Disabled,     //< The Cortex-M4 core is disabled: power off, clock off, isolate, reset and no retain.   
//     CM4Reset,        //< The Cortex-M4 core is in the Reset mode: clock off, no isolated, no retain and reset. 
//     CM4Retained,     //< The Cortex-M4 core is retained. power off, clock off, isolate, no reset and retain.   
// }
impl Cpuss {
    #[inline(always)]
    pub fn configure_clocks_cm4(&self, fast_div: u8) -> () {
        self.cpu_sys
            .cm4_clock_ctl
            .modify(|_, w| unsafe { w.fast_int_div().bits(fast_div) });
    }
    #[inline(always)]
    pub fn configure_clocks_cm0(&self, peri_div: u8, slow_div: u8) -> () {
        self.cpu_sys
            .cm0_clock_ctl
            .modify(|_, w| unsafe { w.peri_int_div().bits(peri_div) });
        self.cpu_sys
            .cm0_clock_ctl
            .modify(|_, w| unsafe { w.slow_int_div().bits(slow_div) });
    }
    ///
    #[inline(always)]
    pub fn cm4_status(&self) -> PWR_MODE_A{
        self.cpu_sys.cm4_pwr_ctl.read().pwr_mode().variant()
    }
    //CM4_reset should only be used by higher level HAL functions that
    // can ensure the pre-requisites around safety can be enforced.
    #[inline(always)]
    pub fn cm4_reset(&self) ->PWR_MODE_A{
        //WARNING: do not call the function while the Cortex-M4 is executing because
        // such a call may corrupt/abort a pending bus-transaction by the CPU and cause
        // unexpected behavior in the system including a deadlock. Call the function
        // while the Cortex-M4 core is in the Sleep or Deep Sleep low-power mode.
        //TODO: when the sleep functions are implemented put a guard on this to
        //enforce no change unless the cpu power_mode is suitable.
        free(|_cs| {
            //Safety: bits are as for TRM pp334 0x05fa is required to unlock the
            //        register so the write will modify the pwr_mode bit.
            self.cpu_sys.cm4_pwr_ctl.modify(|_,w| unsafe{w.bits(0x05fa0001)}); 
            
            //wait for it to fire up.
            while !self.cpu_sys.cm4_status.read().pwr_done().bit_is_set(){}
            
        });
       self.cm4_status()
    }
    //CM4_retain should only be used by higher level HAL functions that
    // can ensure the pre-requisites around safety can be enforced.
     #[inline(always)]
    pub fn cm4_retain(&self) -> PWR_MODE_A{
        //WARNING: do not call the function while the Cortex-M4 is executing because
        // such a call may corrupt/abort a pending bus-transaction by the CPU and cause
        // unexpected behavior in the system including a deadlock. Call the function
        // while the Cortex-M4 core is in the Sleep or Deep Sleep low-power mode.
        //TODO: when the sleep functions are implemented put a guard on this to
        //enforce no change unless the cpu power_mode is suitable.
         free(|_cs| {
            //Safety: bits are as for TRM pp334 0x05fa is required to unlock the
            //        register so the write will modify the pwr_mode bit.
            self.cpu_sys.cm4_pwr_ctl.modify(|_,w| unsafe{w.bits(0x05fa0002)}); 
         });
        self.cm4_status()
    }
    //CM4_disable should only be used by higher level HAL functions that
    // can ensure the pre-requisites around safety can be enforced.
     #[inline(always)]
    pub fn cm4_disable(&self) -> PWR_MODE_A{
        //WARNING: do not call the function while the Cortex-M4 is executing because
        // such a call may corrupt/abort a pending bus-transaction by the CPU and cause
        // unexpected behavior in the system including a deadlock. Call the function
        // while the Cortex-M4 core is in the Sleep or Deep Sleep low-power mode.
        //TODO: when the sleep functions are implemented put a guard on this to
        //enforce no change unless the cpu power_mode is suitable.
         free(|_cs| {
            //Safety: bits are as for TRM pp334 0x05fa is required to unlock the
            //        register so the write will modify the pwr_mode bit.
             self.cpu_sys.cm4_pwr_ctl.modify(|_,w| unsafe{w.bits(0x05fa0002)});
             //wait for it to stop.
            while !self.cpu_sys.cm4_status.read().pwr_done().bit_is_set(){}
         });
        self.cm4_status()
    }
    
     #[inline(always)]
    pub fn cm4_enable(&self) -> PWR_MODE_A{
         free(|_cs| {
            //Safety: bits are as for TRM pp334 0x05fa is required to unlock the
            //        register so the write will modify the pwr_mode bit.
             self.cpu_sys.cm4_pwr_ctl.modify(|_,w| unsafe{w.bits(0x05fa0003)});
             //wait for it to start.
            while !self.cpu_sys.cm4_status.read().pwr_done().bit_is_set(){}
         });
        self.cm4_status()
    }
}
