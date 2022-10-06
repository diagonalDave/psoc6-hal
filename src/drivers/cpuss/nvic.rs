//! drivers/nvic.rs provides access to the interrupt nvic
//! functions from the PAC.
use cortex_m::peripheral::NVIC;

#[cfg(feature = "cm0")]
use crate::drivers::cpuss::interrupt::InterruptSource;

#[cfg(feature = "cm4")]
use crate::pac::Interrupt;

pub struct Nvic{
    nvic: NVIC,
}

impl Nvic{ 
    fn new(nvic: NVIC)-> Self{
        Self{
            nvic,
        }
    }
    #[cfg(feature="cm0")]
    #[inline]
    pub unsafe fn configure_interrupt(&mut self, irqn: InterruptSource, priority:u8)-> (){
        
        // # Safety: Unsafe with:
        //  - priority based critical sections.
        //  - mask based critical sections.
        //  otherwise safe.
        //Configure NVIC for
        self.nvic.set_priority(irqn, priority);
        //enable interrupt using cortex_m
        NVIC::unmask(irqn);

        //release any pending interrupts for source.
        self.clear_interrupt(irqn);
    }
    #[cfg(feature="cm0")]
    #[inline]
    fn clear_interrupt(&self, irqn: InterruptSource) ->(){
        NVIC::unpend(irqn);
    }

    
    /// # Safety: Unsafe with:
    ///  - priority based critical sections.
    ///  - mask based critical sections.
    //  otherwise safe.
    #[cfg(feature="cm4")]
    #[inline]
    pub unsafe fn configure_interrupt(&mut self, irqn: Interrupt, priority:u8)-> (){
        
        
        
        //Configure NVIC for
        self.nvic.set_priority(irqn, priority);
        //enable interrupt using cortex_m
        NVIC::unmask(irqn);
        
        //release any pending interrupts for source.
        self.clear_interrupt(irqn);
    }
    #[cfg(feature="cm4")]
    #[inline]
    pub fn clear_interrupt(&self, irqn: Interrupt) ->(){
        NVIC::unpend(irqn);
    }
    
}
impl core::convert:: From<NVIC> for Nvic{
    fn from(nvic: NVIC)->Self{
        Self::new(nvic)
    }
}

