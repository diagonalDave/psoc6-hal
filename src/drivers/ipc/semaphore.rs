// ipc/semaphore.rs implements a semaphore system for the ipc
// module. The semaphore model is implemented as bit fields on
// u128.
// The semaphore model requires each core to create a local
// semaphore struct. These copies are kept in sync using the
// semaphore IPC channel.
// Usage:
//   1. Create a local semaphore for each core.
//   2. Create data to send on IPC channel, say ep0.
//   3. To send data via ep0:
//      1. Acquire ep0.
//      2. Load data register.
//      3. Set a semaphore flag for the ep0 data read.
//      4. Notify other core ep0 has new data.
//   4. Reciever (has previously initialised local semaphore) then:
//      1. Creates a local SemaphoreFlag<Set> with the data from the
//         semaphore channel, by calling notify_set (which releases
//         the semaphore channel).
//      2. Reads the ep0 data.
//      3. Releases the ep0 channel.
//      4. Completes copying/using the ep0 data.
//      5. Calls release on the local copy of the SemaphoreFlag<Set>
//   5. Sender then uses the release notification to call
//      release_notify to update the Semaphores local copy.
//   6. Releases the Semaphore channel.

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
#[derive(Debug)]
pub struct SemaphoreFlag<FLAG> {
    _flag: PhantomData<FLAG>,
    pub flag: u32,
}


pub struct Set {}
pub struct Clear {}

pub struct Configured{}
pub struct UnInit{}

pub trait State {}
impl State for Configured{}
impl State for UnInit{}
impl State for Set {}
impl State for Clear{}

impl<S>   Semaphore<S>{
    pub fn new()-> Semaphore<UnInit>{
        Semaphore{
            flags: 0,
            release_mask: IntrStructMaskBits::none,
            notify_mask: IntrStructMaskBits::none,
            _state: PhantomData::<UnInit>,
        }
    }
   
}

impl<'a> Semaphore<UnInit>{
    #[cfg(not(armv7em))]
    pub fn configure(&self, channel: &'a mut Semaphores<Released>, intr_struct: &'a mut Syscall<Released>, config: ChannelConfig) -> Result<Semaphore<Configured>, Error> {
        //create IPC channel
        let mut acquired_channel = channel.acquire_lock()?;
        free(|cs| {            
            intr_struct.configure_notify(&config.intr_notify_mask, cs);
            intr_struct.configure_release(&config.intr_release_mask, cs);            
        });
        //Send the semaphore pointer to the CM4 core.
        
        unsafe{acquired_channel.write_data_register(core::ptr::addr_of!(self) as *const u32)};
        //Don't wait for other channel to unlock given this could be run
        //during system startup well before CM4 has started.
        acquired_channel.release_lock(&IntrStructMaskBits::none)?;
        Ok(Semaphore {
            flags: self.flags,
            release_mask: config.struct_release,
            notify_mask: config.struct_notify,
            _state: PhantomData::<Configured>,
        })
    }
    ///The CM4 core 
    #[cfg(armv7em)]
    pub fn configure<L: Lock>(self, channel: &'a mut Semaphores<L> ) -> Semaphore<Configured> {
        //Read the data present on Semaphores IpcChannel.
        let temp_sen: * const Semaphore<Configured>;
//      unsafe{
            temp_sen = channel.read_data_register() as *const Semaphore<Configured>;
//      }
        //Channel shouldn't be locked if being configured.
        //Releasing without notification just in case.
        channel.release_lock(&IntrStructMaskBits::none);
        Semaphore{
            flags: unsafe{(*temp_sen).flags},
            release_mask: unsafe{(*temp_sen).release_mask},
            notify_mask: unsafe{(*temp_sen).notify_mask} ,
            _state: PhantomData::<Configured>
            }

    }
}

impl<'a> Semaphore<Configured>{
    pub fn set(&mut self, flag_number: u32) -> Result<(), Error> {
        self.set_local(flag_number)       
    }
   

    pub fn clear(&mut self, flag_number: u32) -> Result<(), Error> {
        self.clear_local(flag_number)
    }
    #[inline(always)]
    pub fn flag_is_set(&self, flag_number: u32) -> bool{
        self.flags & (!(1 << flag_number)) != 0
    }

    fn clear_local(&mut self, flag_number: u32) -> Result<(), Error> {
        let mask = !(1 << flag_number);
        if mask & self.flags != 0 {
            self.flags &= !(1 << flag_number);
            Ok(())
        }else{
            Err(Error::FlagCannotBeClearedIsNotSet)
        }
     }

    fn set_local(&mut self, flag_number: u32) -> Result<(), Error> {
        if flag_number < 127 {
            if self.flags & (1 << flag_number) != 0 {
                Err(Error::FlagLocked)
            } else {
                self.flags |= 1 << flag_number;
                Ok(())
            }
        } else {
            Err(Error::FlagUnknown)
        }
    }
    
}
