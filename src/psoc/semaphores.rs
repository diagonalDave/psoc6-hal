use crate::drivers::ipc::semaphore::{
    Semaphore,
    Error,
    SemaphoreFlag,
    Set, Clear
};

use crate::drivers::ipc::{
    MaskBits,
    ChannelConfig,    
};
use crate::psoc::Psoc;

impl Psoc{
    //configure_semaphores sets up the ipc Semaphores channel and configures
    //any required state.
    // The semaphores are used by the system to control data handling for
    // data structures passed around in the IPC system.
    pub fn configure_ipc_channel_intr(&self, ChannelConfig)->Result<(),Error>{
        todo!()
        let sem_config = ChannelConfig{release_mask:MaskBits::struct4, notify_mask:MaskBits::struct4};
        self.ipc.semaphores.configure_channel(sem_config);
    }
    pub fn send_ipc_data(&self, data: *const u32) -> Result<Semaphore<Set>, Error>{
        todo!()
