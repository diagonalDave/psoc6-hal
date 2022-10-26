//! drivers/nvic.rs provides access to the interrupt nvic
//! functions from the PAC.
use crate::pac::NVIC;

use crate::drivers::cpuss::interrupt::InterruptSource;

pub struct Nvic {
    nvic: NVIC,
}

impl Nvic {
    fn new(nvic: NVIC) -> Self {
        Self { nvic }
    }
    /// # Safety: Unsafe with:
    ///  - mask based critical sections.
    ///  otherwise safe.
    #[inline]
    pub unsafe fn enable_interrupt(&mut self, irqn: InterruptSource) -> () {
        //Configure NVIC for
        NVIC::unmask(irqn);
    }
    #[inline]
    pub fn disable_interrupt(&mut self, irqn: InterruptSource) -> () {
        NVIC::mask(irqn);
    }
    #[inline]
    /// # Safety: Unsafe with:
    ///  - priority based critical sections.
    //  otherwise safe.
    pub unsafe fn configure_interrupt(&mut self, irqn: InterruptSource, priority: u8) -> () {
        //Configure NVIC for
        self.nvic.set_priority(irqn, priority);
        //release any pending interrupts for source.
        self.clear_interrupt(irqn);
    }
    #[inline]
    fn clear_interrupt(&self, irqn: InterruptSource) -> () {
        NVIC::unpend(irqn);
    }
}
impl core::convert::From<NVIC> for Nvic {
    fn from(nvic: NVIC) -> Self {
        Self::new(nvic)
    }
}
