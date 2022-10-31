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
    flags_ptr: *const u32,
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

impl<'a, S>   Semaphore<S>{
    #[cfg(not(armv7em))]
    pub fn configure(flags:&'a mut u128, channel: &'a mut Semaphores<Released>, intr_struct: &'a mut Syscall<Released>, config: ChannelConfig) -> Result<Semaphore<Configured>, Error>{
        //Configure the intr_struct notify and release masks.
        let mut acquired_channel = channel.acquire_lock()?;
        free(|cs| {            
            intr_struct.configure_notify(&config.intr_notify_mask, cs);
            intr_struct.configure_release(&config.intr_release_mask, cs);            
        });
        let flags_ptr = core::ptr::addr_of!(*flags) as *const u32;
        unsafe{acquired_channel.write_data_register(flags_ptr)};
        acquired_channel.release_lock(&IntrStructMaskBits::none)?;
         Ok(Semaphore {
             flags_ptr,
             release_mask: config.struct_release,
             notify_mask: config.struct_notify,
             _state: PhantomData::<Configured>,
        })
    }
    ///The CM4 core 
    #[cfg(armv7em)]
    pub fn configure<L: Lock>(channel: &'a mut Semaphores<L>, config: &ChannelConfig ) -> Semaphore<'a, Configured> {
                
        //Channel shouldn't be locked if being configured.
        //Releasing without notification just in case.
        channel.release_lock(&IntrStructMaskBits::none);
        Semaphore{
            flags_ptr: unsafe{channel.read_data_register() as * const u32},
            release_mask: config.struct_release,
            notify_mask: config.struct_notify,
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
        unsafe{*self.flags_ptr as u128} & ((1 << flag_number) as u128) != 0
    }

    fn clear_local(&mut self, flag_number: u32) -> Result<(), Error> {
        let mask = !(1 << flag_number);
        if mask & (*self.flags_ptr as u128) == 0 {
            *self.flags_ptr &= !(1 << flag_number);
            Ok(())
        }else{
            Err(Error::FlagCannotBeClearedIsNotSet)
        }
    }
    // Step thorug cM0
    // 0x4023008c	9c	3e	02	08 sets semaphore fine.
    // little endian   08023e9c -> 04	03	32	40                 
    // 802 3F98  0x0000_0000_0000_0001_0000_0000_0000_0000
    //Step through on Cm4
    //0x4023008c	9c	3e	02	08 same as cm0 so it appears pointer is passed correctly.
    //sem 0x080477B0 (All)	semaphore not set to shared semaphore memory address.

    //0x0000_0001_0000_0001_0000_0000_0000_0000
    
    fn set_local(&mut self, flag_number: u32) -> Result<(), Error> {
        if flag_number < 127 {
            if *self.flags & (1 << flag_number) != 0 {
                Err(Error::FlagLocked)
            } else {
                *self.flags |= 1 << flag_number;
                Ok(())
            }
        } else {
            Err(Error::FlagUnknown)
        }
    }
    
}
