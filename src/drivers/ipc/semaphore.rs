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
    Semaphores,
    IntrStructMaskBits,
    ChannelConfig,
    Syscall,
    Lock,
};

pub struct Semaphore<'a, LOCK, INTRLOCK, STATE> {
    flags: u128,
    channel: &'a Semaphores<LOCK>,
    intr_struct: &'a Syscall<INTRLOCK>,
    _state: PhantomData<STATE>,
}
#[derive(Debug)]
pub struct SemaphoreFlag<FLAG> {
    _flag: PhantomData<FLAG>,
    pub flag: u32,
}
#[derive(Debug)]
pub enum Error {
    FlagUnknown,
    FlagLocked,
    FlagNotSet,
}

pub struct Set {}
pub struct Clear {}

trait Flag {}
impl Flag for Set {}
impl Flag for Clear {}

pub struct Configured{}
pub struct UnInit{}

trait State {}
impl State for Configured{}
impl State for UnInit{}

impl<'a, L, IL, S>  Semaphore<'a, L, IL, S>
where
    L: Lock,
    IL: Lock,
    S: State{
    pub fn new(channel: &'a Semaphores<L>, intr_struct: &'a Syscall<IL> ) -> Semaphore<L, IL, UnInit> {
        //create IPC channel
        Semaphore {
            flags: 0u128,
            channel,
            intr_struct,
            _state: PhantomData::<UnInit>,
        }
    }
    pub fn configure(self, config: & ChannelConfig) -> Semaphore<Released, Released, Configured>{
        let channel = self.channel.acquire_lock?;
        free(|cs| {            
            self.intr_struct.configure_notify(&config.intr_notify_mask, cs);
            self.intr_struct.configure_release(&config.intr_release_mask, cs);
        });
        channel.release_lock(IntrStructMaskBits::none);
        Semaphore {
            flags: self.flags,
            channel: self.channel,
            intr_struct: self.intr_struct, 
            _state: PhantomData::<Configured>,
        }
    }
}
impl<'a, L, IL, S>  Semaphore<'a, L, IL, Configured>
where
    L: Lock,
    IL: Lock,
    S: State{
    pub fn set(&mut self, flag_number: u32) -> Result<SemaphoreFlag<Set>, Error> {
        self.set_local(flag_number)?;
        
        //Acquire the Semaphore channel
        self.acquire_lock()?;
        
        //Send the data.
        //#Safety: atomic write synchronised with other core by lock.
        unsafe{self.channel.write_data_register(flag_number)};
        
        //Send a notification to the other core.
        self.channel.notify(&IntrStructMaskBits::syscall);
        
        Ok(SemaphoreFlag{
            _flag: PhantomData::<Set>,
            flag: flag_number,
        })
    }
    /// notify_clear clears the semaphore flag after a notification has
    /// been received.
    pub fn notify_clear(&mut self) -> Result<SemaphoreFlag<Set>, Error> {
        // Read the Semaphore sent data.
        // Release the Semaphore channel.
        // Update local copy
        //Return a Set semaphore flag.
        self.set_local(flag)
    }

    pub fn clear(&mut self, flag: SemaphoreFlag<Set>) -> SemaphoreFlag<Clear> {
        let data = self.read_data_register(); 
        //Acquire the Semaphore channel
        //Send the semaphoreFlag as data.
        //Send a notification.
        //clear the flag
        self.clear_local(flag)
    }
    pub fn notify_clear(&mut self, flag: SemaphoreFlag<Set>) -> SemaphoreFlag<Clear> {
        //Read the Semaphore data
        // Release the Semaphore channel
        // Update the local Semaphore copy.
        //Return a SemaphoreFlag<Clear>
        self.clear_local(flag)
    }

    fn clear_local(&mut self, flag: SemaphoreFlag<Set>) -> SemaphoreFlag<Clear> {
        self.flags &= !(1 << flag.flag);
        //create a clear semaphoreFlag.
        SemaphoreFlag {
            _flag: PhantomData::<Clear>,
            flag: flag.flag,
        }
    }

    fn set_local(&mut self, flag_number: u32) -> Result<SemaphoreFlag<Set>, Error> {
        if flag_number < 127 {
            if self.flags & (1 << flag_number) != 0 {
                Err(Error::FlagLocked)
            } else {
                self.flags |= 1 << flag_number;
                Ok(SemaphoreFlag {
                    _flag: PhantomData::<Set>,
                    flag: flag_number,
                })
            }
        } else {
            Err(Error::FlagUnknown)
        }
    }
}
