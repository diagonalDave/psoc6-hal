//! ipc/mod.rs implements inter-core communication using the
//! inter-process communication (IPC) registers.


use core::marker::PhantomData;
use crate::pac::IPC;

use bitflags::bitflags;

pub trait IpcChannel{
    type Channels;
    fn split(self) -> Self::Channels;
}
#[derive(Debug)]
pub struct Acquired;
#[derive(Debug)]
pub struct Released;
pub trait Lock {}
impl Lock for Acquired {}
impl Lock for Released {}

#[derive(Debug)]
pub enum Error {
    AcquisitionFailed,
    ReleaseFailed,
    SendFailed,
    ReceiveFailed,
}

bitflags!{
    #[derive(Debug)]
    pub struct MaskBits:u16 {
        const struct0 = (1 << 0);
        const struct1 = (1 << 1);
        const struct2 = (1 << 2);
        const struct3 = (1 << 3);
        const struct4 = (1 << 4);
        const struct5 = (1 << 5);
        const struct6 = (1 << 6);
        const struct7 = (1 << 7);
        const struct8 = (1 << 8);
        const struct9 = (1 << 9);
        const struct10 = (1 << 10);
        const struct11 = (1 << 11);
        const struct12 = (1 << 12);
        const struct13 = (1 << 13);
        const struct14 = (1 << 14);
        const struct15 = (1 << 15);
    }
}
#[derive(Debug)]
pub struct ChannelConfig{
    pub release_mask: MaskBits,
    pub notify_mask: MaskBits,
}

macro_rules! ipc{
    ([
        $($C:ident: ($c:ident, $structi:ident, $intr_structi:ident, $LOCK:ty)),+
    ]) => {

        //Create the channels
        #[derive(Debug)]
        pub struct Channels{
            $(
                //Channel
                pub $c: $C<$LOCK>,
            )+
        }
        impl IpcChannel for IPC {
            type Channels = Channels;
            fn split(self) -> Channels {
                Channels {
                    $(
                        $c: $C { _lock: PhantomData},
                    )+
                }
            }
        }
        $(
            ///Channel
            #[derive(Debug)]
            pub struct $C<LOCK> {
                _lock: PhantomData<LOCK>,
            }
            impl<LOCK> $C<LOCK>{
                ///Configures the channel to start released.
                pub fn into_released(self)->$C<Released>{
                    $C{ _lock: PhantomData }
                }
            }
            impl $C<Released>{
                /// configure_channel configures the release and notify
                /// interrupt structures for the channel.
                /// IPC channels send notify and release events to an IPC interrupt
                /// structure, the structure then forwards these events to the
                /// the interrupt controller and an Interrupt line such as 
                /// CPUSS_INTERRUPTS_IPC_1 where it can be handled via a software
                /// ISR (although this an unlikely method to manage IPC).
                /// The interrupts can also be managed directly at the register
                /// level:
                /// - Read the MASKED register to find any active interrupts for
                ///   this channel.
                /// - Write to the SET register to activate an interrupt
                /// - Write to the INTR register to clear an interrupt
                pub fn configure_channel(self, config: ChannelConfig)-> Self{
                    unsafe{
                        (*IPC::PTR)
                            .$intr_structi
                            .intr_mask
                            .write(|w|
                                    w
                                    .notify()
                                    .bits(config.notify_mask.bits()));                   
                        (*IPC::PTR)
                            .$intr_structi
                            .intr_mask
                            .write(|w|
                                    w
                                    .release()
                                   .bits(config.release_mask.bits()));

                        Self{_lock: PhantomData::<Released>}
                    }
                }
                // #Safety - Single instruction read.
                #[inline]
                fn acquire_lock(self) ->Result<$C<Acquired>, Error> {
                    let mut count:u8 = 0;
                    while unsafe{(*IPC::PTR)
                                 .$structi
                                 .acquire
                                 .read()
                                 .success()
                                 //arbitrary timeout of 200 attempts.
                                 .bit_is_clear() }&& count < 200{count += 1;}
                    if count < 200 {
                        Ok($C {
                            _lock: PhantomData::<Acquired>,
                        })
                    }else{
                        Err(Error::AcquisitionFailed)
                    }
                }
                /// send_data_ptr is used for sending more than one
                /// byte of data between channels.
                /// Input args are:
                /// - a pointer to the data structure to send.
                /// - a bit mask to set the notify interrupt
                ///   (configured with configure interrupts function).
                /// A Result is returned with a Released lock or an Error enum.
                /// #Safety - Safe locked single instruction write to IPC PTR.
                pub fn send_data_ptr(self, data: *const u32, notify: MaskBits ) -> Result<$C<Released>, Error>{
                    let data_lock = self.acquire_lock()?;
                    unsafe{
                        (*IPC::PTR)
                            .$structi
                            .data
                            .write(|w| w.bits(data as u32))
                    }
                    let mut data_unlocked = data_lock.release_lock()?;
                    data_unlocked.notify(notify);
                    Ok(data_unlocked)
                    
                }
                /// recieve_data_ptr is used for receiving more than one
                /// byte of data from a channel.
                /// Input args are:
                /// - a pointer to the data structure to send.
                /// - a bit mask to clear the release interrupt
                ///   (configured with configure interrupts function).
                /// A Result is returned with a tuple of a (Released lock, data pointer) or an Error enum.
                pub fn receive_data_ptr(self, release: MaskBits) -> Result<($C<Released>, *const u32) , Error>{
                     let data_lock = self.acquire_lock()?;
                     let ptr: *const u32;
                     unsafe{
                         ptr = (*IPC::PTR)
                             .$structi
                             .data
                             .read()
                             .bits() as *const u32
                     }
                     let mut data_unlocked = data_lock.release_lock()?;
                     data_unlocked.clear_release(release);
                    Ok((data_unlocked, ptr))
                    
                 }
                /// send_data_byte requires a Released channel and a data byte
                /// to transfer to the other CPU.
                /// The return value is a  Released channel  or an Error.
                //  
                pub fn send_data_byte(self, data: u32, notify:MaskBits ) -> Result<$C<Released>, Error>{
                    
                    let data_lock = self.acquire_lock()?;
                    unsafe{
                        (*IPC::PTR)
                            .$structi
                            .data
                            .write(|w| w.bits(data))
                    }
                    let mut  data_unlocked = data_lock.release_lock()?;
                    data_unlocked.notify(notify);
                    Ok(data_unlocked)
                    
                 }

                 pub fn receive_data_byte(self, release: MaskBits) -> Result<($C<Released>, u32) , Error>{
                     let data_lock = self.acquire_lock()?;
                     let data: u32;
                     unsafe{
                         data = (*IPC::PTR)
                             .$structi
                             .data
                             .read()
                             .bits()
                     }
                     let mut data_unlocked = data_lock.release_lock()?;
                     data_unlocked.clear_release(release);
                    Ok((data_unlocked, data))
                    
                 }
                /// notify is used mainly for transferring data.
                /// Writing the notify bits sends an event to the set 
                /// interrupt structures that fire an interrupt. Indicating
                /// to the notify channels channel that data is ready.
                 #[inline(always)]
                fn notify(&mut self, notify_set: MaskBits) ->() {
                    unsafe{(*IPC::PTR)
                           .$intr_structi
                           .intr_set
                           .write(|w|
                                  w.notify()
                                  .bits(notify_set.bits()))
                    }

                }
                /// release is used by software to set the interrupt bit for each of the
                /// notify_set bits. This fires the release interrupt for any of the configured
                /// channels. Typically this is left to the hardware.
                 #[inline(always)]
                fn release(&mut self, notify_set: MaskBits) ->() {
                    unsafe{(*IPC::PTR)
                           .$intr_structi
                           .intr_set
                           .write(|w|
                                  w.notify()
                                  .bits(notify_set.bits()))
                    }

                }
                /// clear_release clears any release interrupts for each of the release_set bits.
                #[inline(always)]
                fn clear_release(&mut self, release_set: MaskBits)->(){
                    unsafe{(*IPC::PTR)
                           .$intr_structi
                           .intr
                           .write(|w|
                                  w.release()
                                  .bits(release_set.bits()))
                    }
                }
                /// clear_notifications clears any notification interrupts for each of the notify_set bits.
                #[inline(always)]
                fn clear_notify(&mut self, notify_set: MaskBits)->(){
                    unsafe{(*IPC::PTR)
                           .$intr_structi
                           .intr
                           .write(|w|
                                  w.notify()
                                  .bits(notify_set.bits()))
                    }
                }
            }
            impl $C<Acquired>{
                #[inline(always)]
                fn release_lock(self) ->Result<$C<Released>, Error> {
                    if unsafe{
                        (*IPC::PTR)
                            .$structi
                            .acquire
                            .read()
                            .success()
                            .bit_is_clear()
                    }{
                        Ok($C {
                            _lock: PhantomData::<Released>,
                        })
                    }else{
                        Err(Error::ReleaseFailed)
                    }
                }
               
            }
        )+
  
    };
}

ipc!([
    SyscallCm0: (syscall_cm0, struct0, intr_struct0, Released),
    SyscallCm4: (syscall_cm4, struct1, intr_struct1, Released),
    SyscallDap: (syscall_dap, struct2, intr_struct2, Released),
    Semaphores: (semaphores, struct4, intr_struct4, Released),
    PipeEp0: (pipe_ep0, struct5, intr_struct5, Released),
    PipeEp1: (pipe_ep1, struct6, intr_struct6, Released),

    Ddft: (ddft, struct7, intr_struct7, Released),
    Channel8: (channel8, struct8, intr_struct8, Released),
    Channel9: (channel9, struct9, intr_struct9, Released),
    Channel10: (channel10, struct10, intr_struct10, Released),
    Channel11: (channel11, struct11, intr_struct11, Released),
    Channel12: (channel12, struct12, intr_struct12, Released),
    Channel13: (channel13, struct13, intr_struct13, Released),
    Channel14: (channel14, struct14, intr_struct14, Released),
    Channel15: (channel15, struct15, intr_struct15, Released)
    ]);
