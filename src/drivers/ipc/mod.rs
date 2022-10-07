//! ipc/mod.rs implements inter-core communication using the
//! inter-process communication (IPC) registers.


use core::marker::PhantomData;
use crate::pac::IPC;
use bitfield;

pub trait IpcChannel{
    type Channels;
    fn split(self) -> Self::Channels;
}
pub struct Acquired;
pub struct Released;
pub trait Lock {}
impl Lock for Acquired {}
impl Lock for Released {}

pub enum Error {
    AcquisitionFailed,
    ReleaseFailed,
    SendFailed,
    ReceiveFailed,
}

bitfield!{
    pub struct ChannelConfig(u16);
    impl Debug;
    pub mask_release, set_mr: 15, 0;  //0x0000_ffff
    pub mask_notify, set_mn: 31, 16;   // 0xffff_0000
    pub set_release, set_sr: 47, 32;
    pub set_notify, set_sn: 63, 48;
}

macro_rules! ipc{
    ([
        $($C:ident: ($c:ident, $structi:ident, $intr_structi:ident, $LOCK:ty)),+
    ]) => {

        //Create the channels
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
                                    .bits(config.mask_notify));                   
                        (*IPC::PTR)
                            .$intr_structi
                            .intr_mask
                            .write(|w|
                                    w
                                    .release()
                                   .bits(config.mask_release));

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
                ///

                pub fn send_data_ptr(self, data: *const u32 ) -> Result<$C<Released>, Error>{
                    let data_lock = self.acquire_lock()?;
                    unsafe{
                        (*IPC::PTR)
                            .$structi
                            .data
                            .write(|w| w.bits(data as u32))
                    }
                    let data_unlocked = data_lock.release_lock()?;
                    Ok(data_unlocked)
                    
                }

                 pub fn receive_data_ptr(self) -> Result<($C<Released>, *const u32) , Error>{
                     let data_lock = self.acquire_lock()?;
                     let ptr: *const u32;
                     unsafe{
                         ptr = (*IPC::PTR)
                             .$structi
                             .data
                             .read()
                             .bits() as *const u32
                     }
                    let data_unlocked = data_lock.release_lock()?;
                    Ok((data_unlocked, ptr))
                    
                 }
                /// send_data_byte requires a Released channel and a data byte
                /// to transfer to the other CPU.
                /// The return value is a  Released channel  or an Error.
                //  
                pub fn send_data_byte(self, data: u32, config: ChannelConfig ) -> Result<$C<Released>, Error>{
                    
                    let data_lock = self.acquire_lock()?;
                    unsafe{
                        (*IPC::PTR)
                            .$structi
                            .data
                            .write(|w| w.bits(data))
                    }
                    data_lock.release_lock()?
                        .notify(config.set_notify_mask);
                    //TODO:data_unlocked.release_notify();
                    
                    Ok(data_unlocked)
                    
                 }

                 pub fn receive_data_byte(self) -> Result<($C<Released>, u32) , Error>{
                     let data_lock = self.acquire_lock()?;
                     let data: u32;
                     unsafe{
                         data = (*IPC::PTR)
                             .$structi
                             .data
                             .read()
                             .bits()
                     }
                    let data_unlocked = data_lock.release_lock()?;
                    Ok((data_unlocked, data))
                    
                 }
                 #[inline(always)]
                fn notify(self, notify_bits: u16) ->() {
                    unsafe{(*IPC::PTR)
                           .$structi
                           .notify
                           .write(|w|
                                  w.intr_notify()
                                  .bits(notify_bits))
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
    SyscallCm4: (syscall_cm4, struct1, intr_struct1, Released)
    // SyscallDap: (syscall_dap, struct2),
    // Semaphores: (semaphores, struct4),
    // PipeEp0: (pipe_ep0, struct5),
    // PipeEp1: (pipe_ep1, struct6),
    // Ddft: (ddft, struct7),
    // Channel8: (channel8, struct8),
    // Channel9: (channel9, struct9),
    // Channel10: (channel10, struct10),
    // Channel11: (channel11, struct11),
    // Channel12: (channel12, struct12),
    // Channel13: (channel13, struct13),
    // Channel14: (channel14, struct14),
    // Channel15: (channel15, struct15)
    ]);
