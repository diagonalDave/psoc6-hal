use crate::drivers::ipc::semaphore::{
    SemaphoreFlag,
    Set,
};

use cortex_m::interrupt::free;
use crate::drivers::ipc::{
    IntrStructMaskBits,
    ChannelConfig,
};
use crate::psoc::Psoc;
use crate::error::Error;
impl Psoc{
    //configure_ipc_semaphores sets up the ipc Semaphores channel and configures
    //any required state.
    // The semaphores are used by the system to control data handling for
    // data structures passed around in the IPC system.
   
    pub fn configure_ipc_semaphore(&mut self, channel_config: &ChannelConfig )->Result<(), Error>{
        //acquire lock as a flag atm.
        let mut sema_channel = self.ipc.semaphores.acquire_lock()?;
        free(|cs| { 
        match channel_config.struct_release { 
            IntrStructMaskBits::syscall          => self.ipc_intr.syscall.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_struct1     => self.ipc_intr.intr_struct1.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_struct2     => self.ipc_intr.intr_struct2.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_ep0         => self.ipc_intr.intr_ep0.configure_release(&channel_config.intr_release_mask, cs),    
            IntrStructMaskBits::intr_ep1         => self.ipc_intr.intr_ep1.configure_release(&channel_config.intr_release_mask, cs),    
            IntrStructMaskBits::intr_struct5     => self.ipc_intr.intr_struct5.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_struct6     => self.ipc_intr.intr_struct6.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_spare       => self.ipc_intr.intr_spare.configure_release(&channel_config.intr_release_mask, cs),  
            IntrStructMaskBits::intr_struct8     => self.ipc_intr.intr_struct8.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_struct9     => self.ipc_intr.intr_struct9.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_struct10    => self.ipc_intr.intr_struct10.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_struct11    => self.ipc_intr.intr_struct11.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_struct12    => self.ipc_intr.intr_struct12.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_struct13    => self.ipc_intr.intr_struct13.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_struct14    => self.ipc_intr.intr_struct14.configure_release(&channel_config.intr_release_mask, cs),
            IntrStructMaskBits::intr_struct15    => self.ipc_intr.intr_struct15.configure_release(&channel_config.intr_release_mask, cs),
            _ => (),
        };
        match channel_config.struct_notify {
            IntrStructMaskBits::syscall          => self.ipc_intr.syscall.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_struct1     => self.ipc_intr.intr_struct1.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_struct2     => self.ipc_intr.intr_struct2.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_ep0         => self.ipc_intr.intr_ep0.configure_notify(&channel_config.intr_notify_mask, cs),    
            IntrStructMaskBits::intr_ep1         => self.ipc_intr.intr_ep1.configure_notify(&channel_config.intr_notify_mask, cs),    
            IntrStructMaskBits::intr_struct5     => self.ipc_intr.intr_struct5.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_struct6     => self.ipc_intr.intr_struct6.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_spare       => self.ipc_intr.intr_spare.configure_notify(&channel_config.intr_notify_mask, cs),  
            IntrStructMaskBits::intr_struct8     => self.ipc_intr.intr_struct8.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_struct9     => self.ipc_intr.intr_struct9.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_struct10    => self.ipc_intr.intr_struct10.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_struct11    => self.ipc_intr.intr_struct11.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_struct12    => self.ipc_intr.intr_struct12.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_struct13    => self.ipc_intr.intr_struct13.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_struct14    => self.ipc_intr.intr_struct14.configure_notify(&channel_config.intr_notify_mask, cs),
            IntrStructMaskBits::intr_struct15    => self.ipc_intr.intr_struct15.configure_notify(&channel_config.intr_notify_mask, cs),
            _            => (),
        };  
        });
        sema_channel.release_lock(&channel_config.struct_notify)?; 
        Ok(())
    }
    pub fn send_ipc_semaphore(&mut self, flag:&SemaphoreFlag<Set>, notify_mask: &IntrStructMaskBits)-> Result<(), Error>{
        let sema_lock = self.ipc.semaphores.acquire_lock()?;
        unsafe{ sema_lock.write_data_register(flag.flag)};
        sema_lock.notify( notify_mask);
        while sema_lock.is_locked(){}
        Ok(())
    }
    pub fn receive_ipc_semaphore(&mut self, flag: &SemaphoreFlag<Set>, release_mask:&IntrStructMaskBits) -> Result<u32, Error>{
        let sem_data = self.ipc.semaphores.read_data_register();
        if sem_data == flag.flag {
            Ok(flag.flag as u32)
        }else{
            self.ipc.semaphores.release_lock(release_mask)?;
            Ok(sem_data)
        }
    }
}
