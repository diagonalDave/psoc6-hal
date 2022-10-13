//! startup.rs implments the required functions for starting
//! both the Cm0p and Cm4 cores.
use crate::drivers::system::reset_cause::ResetCause;


use crate::psoc::Psoc;

impl Psoc{
    pub fn start_cm0p(&self)->(){
        /*Setup clocks and dependant peripherals*/
        //set wait states for worst case of hf_clk == 150Mhz
        self.cpuss.configure_wait_states(150_000_000, &self.modes.system_mode);
        self.flash.configure_wait_states(150_000_000, &self.modes.system_mode);

        //check the reset reason to see if backup domain should be reset.
        //not entirely convinced this is correct but is modelled on psoc_creator code.
        match self.system.last_reset(){
            ResetCause::WDTReset |
            ResetCause::NoReset => {
                self.backup.reset();
                self.system.stop_lfclk();
                self.system.start_lfclk();
            },
                _ => ()
        }
        while self.backup.not_running(){}//wait for backup domain to power up.
        
        //LDO--set the system and flash LDO to enable LP system mode.        
        self.system.configure_ldo_mode(&self.modes.system_mode);
        self.flash.configure_ldo_mode(&self.modes.system_mode);

        //configure pmic
        self.backup.disable_pmic();
        
        //configure clocks
        self.cpuss.configure_clocks_cm0(0x01, 0x00); // peri_div, slow_div
        self.cpuss.configure_clocks_cm4(0x00);       // fast_div
        
        //system level clocks
        self.system.configure_system_clocks();
        self.system.start_system_clocks();
        
        //ancillary clock support stuff.
        self.backup.configure_backup_clock_source();
        self.cpuss.configure_systick_source();
        
        //set wait states for the system clocks with hf_clk == 100Mhz
        self.cpuss.configure_wait_states(100_000_000, &self.modes.system_mode); 
        self.flash.configure_wait_states(100_000_000, &self.modes.system_mode);  
        /* End clock setup */
        
        /*Setup IPC */
        //IPC is configured as for Psoc startup code. Maybe needs refactoring
        //once complete.
        self.startup_ipc_system();
       
        
    }
    /// startup_ipc intialises the core communication infrastructure.
    pub(crate)fn startup_ipc_system(&self)-> (){
        //Configure the semaphores channel.
        //
        
        
    }
}
