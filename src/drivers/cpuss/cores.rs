use crate::drivers::cpuss::Cpuss;

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
}
