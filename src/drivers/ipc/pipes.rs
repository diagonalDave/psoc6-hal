//! ipc/pipes.rs provides a pipe interface to the ipc module.

use crate::drivers::ipc::{
    semaphore::Semaphore,
    ChannelConfig,
    ChannelMaskBits,
    Channels,
    InterruptStructMaskBits,
    InterruptMaskBits,
};

pub enum Error {
    PipeEndpointsNotDistinct,
    PipeEp0Busy,
    PipeEp1Busy,
    
}
pub struct Pipe {
    receive_ep: ChannelMaskBits,
    receive_ep_config: ChannelConfig,
    send_ep: ChannelMaskBits,
    send_ep_config: ChannelConfig,
    semaphore: Semaphore,
}

impl Pipe {
    pub fn new() -> Pipe {
        Self {
            receive_ep: ChannelMaskBits::none,
            receive_ep_config: ChannelConfig {
                release_mask: InterruptStructMaskBits::none,
                notify_mask: InterruptStructMaskBits::none,
                intr_release_mask: InterruptMaskBits::none,
                intr_notify_mask: InterruptMaskBits::none,
            },
            send_ep: ChannelMaskBits::none,
            send_ep_config: ChannelConfig {
                release_mask: InterruptStructMaskBits::none,
                notify_mask: InterruptStructMaskBits::none,
                intr_release_mask: InterruptMaskBits::none,
                intr_notify_mask: InterruptMaskBits::none,
            },
            semaphore: Semaphore::new(),
        }
    }
    fn configure_pipe_endpoints(
        self,
        send_ep: ChannelMaskBits,
        receive_ep: ChannelMaskBits,
    ) -> Result<Pipe, Error> {
        if send_ep != receive_ep {
            Ok(Pipe {
                receive_ep,
                send_ep,
                send_ep_config: self.send_ep_config,
                receive_ep_config: self.receive_ep_config,
                semaphore: self.semaphore,
            })
        } else {
            Err(Error::PipeEndpointsNotDistinct)
        }
    }
}
impl Pipe {
    // System pipes are used by the system for inter-core communication. The IPC
    // channels designated for the system pipe are ep0 and ep1.
    // - PipeEp0 is the CM0 core receive endpoint.
    // - PipeEp1 is the CM4 core receive endpoint.
    // Each endpoint is configured to use the equivalently numbered intr_structx:
    // - PipeEp0 uses interrupt structure intr_struct5
    // - PipeEp1 uses interrupt structure intr_struct6
    #[cfg(armv6m)]
    pub fn configure_system_pipe(&mut self, channels: &mut Channels) -> Result<(), Error> {
        // Create the pipe: receives on ep0 channel sends on ep1 channel
        let ep0_cfg = ChannelConfig {
            release_mask: InterruptStructMaskBits::ep1,
            notify_mask: InterruptStructMaskBits::ep1,
            intr_release_mask: InterruptMaskBits::cpuss_interrupt1,
            intr_notify_mask: InterruptMaskBits::cpuss_interrupt1,
        };
        let ep1_cfg = ChannelConfig {
            release_mask: InterruptStructMaskBits::ep0,
            notify_mask: InterruptStructMaskBits::ep0,
            intr_release_mask: InterruptMaskBits::cpuss_interrupt0,
            intr_notify_mask: InterruptMaskBits::cpuss_interrupt0,
        };
        // IpcChannel
        // Interrupt_mask
        // Interrupt
        let epo_channel = channels.pipe_ep0.configure_channel(ep0_cfg).unwrap();
        let ep1_channel = channels.pipe_ep1.configure_channel(ep1_cfg).unwrap();
        
        //configure the interrupt.
        //set priority
        //set multiplexer.
        
        // Client count?
        // Callback function for each client
        // One release callback function.
        Ok(())
    }
    #[cfg(armv7em)]
    pub fn configure_system_pipes(&self) -> () {
        // Create the pipe for the CM4 core.
        //configure ep1 interrupt
        //set priority.
        //setup semaphore.
    }
    pub fn start_system_pipes(&self) -> () {
        todo!()
    }
    pub fn stop_system_pipes(&self) -> () {
        todo!()
    }
}
