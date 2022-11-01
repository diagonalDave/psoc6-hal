// ipc/semaphore.rs implements a semaphore system for the ipc
// module. The semaphore is created during system startup.
// 
// The semaphore model is implemented as bit fields on
// a u128, providing 128 semaphore flags.
// The 128 flags are divided between the 16 (8 system plus 8 user) IPC channels:
// Each channel has 8 flags for use with its clients.
// The semaphore is configured on the CM0 then a reference is passed
// to the CM4.
// Usage:
// ```no_run
// //CM0 code
// //create a ChannelConfig
//  let config = ChannelConfig {
//         //the intr_struct to be set for release events.
//  struct_release: IntrStructMaskBits::intr_struct15,
//        //the intr_struct for notify events.
//  struct_notify: IntrStructMaskBits::intr_struct15,
//       //the interrupt to fire for a release event      
//  intr_release_mask: InterruptMaskBits::cpuss_interrupt15,
//       //the interrupt to fire for a notify event
//  intr_notify_mask: InterruptMaskBits::cpuss_interrupt14,
//     };
//     //create a configured semaphore.
//     let mut sem = Semaphore::<UnInit>::configure(&mut psoc.ipc_intr.syscall,config).unwrap();
//     //start the semaphore to enable sharing with the CM4.
//     sem.start(&mut psoc.ipc.semaphores).unwrap();
//     // set a semaphore flag in this example flag 64 the first avaialble user flag.
//     sem.set(64).unwrap();
// //CM4 code
// //create a configured semaphore that uses the semaphore started bythe CM0 code.
// 
// let sem = Semaphore::<UnInit>::configure(&mut psoc.ipc.semaphores).unwrap();
//```
use core::marker::PhantomData;
use cortex_m::interrupt::free;
use crate::drivers::ipc::{
    Released,
    Lock,
    Semaphores,
    IntrStructMaskBits,
    ChannelConfig,
    Syscall,
};

use crate::error::Error;

#[derive(Debug)]
#[repr(C)]
pub struct Semaphore<STATE> {
    flags: u128,
    release_mask: IntrStructMaskBits,
    notify_mask: IntrStructMaskBits,
    _state: PhantomData<STATE>,
}
//type state declarations
pub struct Set {}
pub struct Clear {}
pub struct Configured{}
pub struct UnInit{}

pub trait State {}
impl State for Configured{}
impl State for UnInit{}
impl State for Set {}
impl State for Clear{}

// The configure functions provide configured semaphores.
// Because the CM0 is booted first it builds the semaphore struct then
// shares it with the CM4 code using the dedicated Semaphores IPC channel.
// The semaphores IPC channel is only used by the configure functions.
impl<'a, S>   Semaphore<S>{
    // CM0 configures the CM0 semaphore.
    #[cfg(not(armv7em))]
    pub fn configure(intr_struct: &'a mut Syscall<Released>, config: ChannelConfig) -> Result<Semaphore<Configured>, Error>{
        //Configure the intr_struct notify and release masks.
        free(|cs| {            
            intr_struct.configure_notify(&config.intr_notify_mask, cs);
            intr_struct.configure_release(&config.intr_release_mask, cs);            
        });
        
          Ok(Semaphore {
             flags: 0,
             release_mask: config.struct_release,
             notify_mask: config.struct_notify,
             _state: PhantomData::<Configured>,
        })
    }
    ///The CM4 core 
    #[cfg(armv7em)]
    pub fn configure<L: Lock>(channel: &'a mut Semaphores<L>) -> Result<&'a Semaphore<Configured>, Error> {
        //Channel shouldn't be locked if being configured.
        //Releasing without notification just in case.       
        let sem =  unsafe{&*(channel.read_data_register() as * const Semaphore<Configured>)};
        channel.release_lock(&IntrStructMaskBits::none)?;
        Ok(sem)
    }
}

impl<'a> Semaphore<Configured>{
    #[cfg(not(armv7em))]
    pub fn start(&mut self, channel: &'a mut Semaphores<Released>)->Result<(), Error>{
        let mut acquired_channel = channel.acquire_lock()?;
        unsafe{acquired_channel.write_data_register((self as *const Semaphore<Configured>) as *const u32)};
        acquired_channel.release_lock(&IntrStructMaskBits::none)?;
        Ok(())
    }
     #[inline(always)]
    pub fn flag_is_set(&self, flag_number: u32) -> bool{
        self.flags & (1 << flag_number)  != 0
    }
    pub fn set(&mut self, flag_number: u32) -> Result<(), Error> {
        if flag_number < 127 {
            if self.flags & (1 << flag_number) != 0 {
                Err(Error::FlagLocked)
            } else {
               self.flags |= 1 << flag_number;
                Ok(())
            }
        } else {
            Err(Error::AttemptingToSetUnknownFlag)
        }
    }
   

    pub fn clear(&mut self, flag_number: u32) -> Result<(), Error> {
         if flag_number < 127 {
        let mask = !(1 << flag_number);
        if mask & self.flags == 0 {
            self.flags &= !(1 << flag_number);
            Ok(())
        }else{
            Err(Error::FlagCannotBeClearedIsNotSet)
        }
        }else{
             Err(Error::AttemptingToClearUnknownFlag)
        }
    }
   
   //  //internal function for checking then clearing flag.
   //  fn clear_local(&mut self, flag_number: u32) -> Result<(), Error> {
   //      if flag_number < 127 {
   //      let mask = !(1 << flag_number);
   //      if mask & self.flags == 0 {
   //          self.flags &= !(1 << flag_number);
   //          Ok(())
   //      }else{
   //          Err(Error::FlagCannotBeClearedIsNotSet)
   //      }
   //      }else{
   //           Err(Error::AttemptingToClearUnknownFlag)
   //      }
   //  }
   // // internal function to check flag then set if not set.    
   //  fn set_local(&mut self, flag_number: u32) -> Result<(), Error> {
   //      if flag_number < 127 {
   //          if self.flags & (1 << flag_number) != 0 {
   //              Err(Error::FlagLocked)
   //          } else {
   //             self.flags |= 1 << flag_number;
   //              Ok(())
   //          }
   //      } else {
   //          Err(Error::AttemptingToSetUnknownFlag)
   //      }
   //  }
    
}
