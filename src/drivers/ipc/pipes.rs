//! ipc/pipes.rs provides a pipe interface to the ipc module.

use crate::pac::NVIC;
use crate::drivers::ipc;
use crate::drivers::ipc::{
    semaphore::Semaphore,
    ChannelConfig,
    ChannelMaskBits,
    Channels,
    InterruptMaskBits,
    IntrStructMaskBits,
    IntrStructs,
};
use crate::drivers::cpuss::{
    nvic,
    interrupt::InterruptSource,
};
pub enum Error {
    PipeOther(ipc::Error),
    PipeEndpointsNotDistinct,
    PipeEp0Busy,
    PipeEp1Busy,
    PipeBusy,
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
                release_mask: IntrStructMaskBits::none,
                notify_mask: IntrStructMaskBits::none,
                intr_release_mask: InterruptMaskBits::none,
                intr_notify_mask: InterruptMaskBits::none,
            },
            send_ep: ChannelMaskBits::none,
            send_ep_config: ChannelConfig {
                release_mask: IntrStructMaskBits::none,
                notify_mask: IntrStructMaskBits::none,
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
    pub fn configure_system_pipe_channels(self, channels: &mut Channels, intr_structs: IntrStructs, nvic: NVIC) -> Result<Pipe, Error> {
        // PSOC-Creator config values:
        // EP0:
        // - IPC intr         0x0060_0000
        // - Intr_struct      0x0000_0300
        // - Ipc Channel      0x0000_0005
        // EP1:
        // - IPC intr         0x0060_0000
        // - Intr_struct      0x0000_0400
        // - Ipc Channel      0x0000_0006
        //
        // On the CM0 EP0 receives data from CM4 and sends data on EP1.
        let ep0_cfg = ChannelConfig {
            release_mask: IntrStructMaskBits::intr_ep1,
            notify_mask: IntrStructMaskBits::intr_ep1,
            intr_release_mask: InterruptMaskBits::cpuss_interrupt1,
            intr_notify_mask: InterruptMaskBits::cpuss_interrupt1,
        };
        let ep1_cfg = ChannelConfig {
            release_mask: IntrStructMaskBits::intr_ep0,
            notify_mask: IntrStructMaskBits::intr_ep0,
            intr_release_mask: InterruptMaskBits::cpuss_interrupt0,
            intr_notify_mask: InterruptMaskBits::cpuss_interrupt0,
        };
        // IpcChannel
        // Interrupt_mask
        // Interrupt
        //#Safety: this is unsafe because there is  no synchronisation for
        // the intr_struct registers. A data race is possible if this is called
        // outside the startup code.
        if let Err(e) = unsafe{intr_structs.intr_ep0.configure(&ep0_cfg)} {
            return Err(Error::PipeOther(e));
        }
        if let Err(e) = unsafe{ intr_structs.intr_ep1.configure(&ep1_cfg)} {
            return Err(Error::PipeOther(e));
        }

        //configure the interrupt.
        //set priority (==1)
        //set multiplexer.( == 1)
        //Only cpuss_interrupt0 for the cm0+
        let nvic = nvic::new();
        nvic.configure_interrupt(InterruptSource::CPUSS_INTERRUPTS_IPC_0, 1); //priority 1 (second highest)
        
        // Client count?
        // Callback function for each client
        // One release callback function.
        Ok(Pipe {
            send_ep: ChannelMaskBits::ep0,
            send_ep_config: ep0_cfg,
            receive_ep: ChannelMaskBits::ep1,
            receive_ep_config: ep1_cfg,
            semaphore: self.semaphore,
        })
    }
    #[cfg(armv6m)]
    //    pub fn configure_system_pipe_callback(&mut self, callbacks: &callback_fn
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
