//! ipc/mod.rs implements inter-core communication using the
//! inter-process communication (IPC) registers.
//! Message passing between cores is implemented using the IPC registers
//! Essentially any messages can be shared but the systems using the
//! IPC are the BLE subsystem and the Flash subsystem
//!

#![allow(dead_code)] // //TODO:Remove this once development of this file complete.
use crate::pac::IPC;
use bitflags::bitflags;
use core::marker::PhantomData;
use cortex_m::interrupt::CriticalSection;

pub mod semaphore;

//pub mod pipes;

pub trait IpcChannel {
    type Channels;
    type IntrStructs;
    fn split(self) -> (Self::Channels, Self::IntrStructs);
}

/// IpcChannelCallback trait must be implemented by any IPC channel
/// when custom actions are required  by client for any IPC channel
/// release or notify events.

pub trait IpcCallback {
    type DataType;
    fn notify_callback(message: Self::DataType) -> ();
    fn release_callback() -> ();
    fn default_release_callback() -> () {}
}

//#[derive(Debug)]
pub struct Acquired;
//#[derive(Debug)]
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
    ChannelBusy,
    SemaphoreError(semaphore::Error),

}
//allows ? operator use for all errors.
impl core::convert::From<semaphore::Error> for Error{
    fn from( residual: semaphore::Error)-> Error{
        Error::SemaphoreError(residual)
    }
}
bitflags! {
    #[derive(Debug, Eq, PartialEq)]
    pub struct InterruptMaskBits:u32 {
        const cpuss_interrupt0 = (1 << 0);
        const cpuss_interrupt1 = (1 << 1);
        const cpuss_interrupt2 = (1 << 2);
        const cpuss_interrupt3 = (1 << 3);
        const cpuss_interrupt4 = (1 << 4);
        const cpuss_interrupt5 = (1 << 5);
        const cpuss_interrupt6 = (1 << 6);
        const cpuss_interrupt7 = (1 << 7);
        const cpuss_interrupt8 = (1 << 8);
        const cpuss_interrupt9 = (1 << 9);
        const cpuss_interrupt10 = (1 << 10);
        const cpuss_interrupt11 = (1 << 11);
        const cpuss_interrupt12 = (1 << 12);
        const cpuss_interrupt13 = (1 << 13);
        const cpuss_interrupt14 = (1 << 14);
        const cpuss_interrupt15 = (1 << 15);
        const none              = (1 >> 1);
    }
}
bitflags! {
    #[derive(Debug, Eq, PartialEq)]
    pub struct IntrStructMaskBits:u32 {
        const syscall         = (1 << 0);
        const intr_struct1     = (1 << 1);
        const intr_struct2     = (1 << 2);
        const intr_ep0         = (1 << 3);
        const intr_ep1         = (1 << 4);
        const intr_struct5     = (1 << 5);
        const intr_struct6     = (1 << 6);
        const intr_spare       = (1 << 7);
        const intr_struct8     = (1 << 8);
        const intr_struct9     = (1 << 9);
        const intr_struct10    = (1 << 10);
        const intr_struct11    = (1 << 11);
        const intr_struct12    = (1 << 12);
        const intr_struct13    = (1 << 13);
        const intr_struct14    = (1 << 14);
        const intr_struct15    = (1 << 15);
        const none        = (1 >> 1);                //the zero option.
    }
}
bitflags! {
    #[derive(Debug, Eq, PartialEq)]
    pub struct ChannelMaskBits:u32 {
        const syscall_cm0 = (1 << 0);            // syscall_cm0
        const syscall_cm4 = (1 << 1);            // syscall_cm4
        const syscall_dap = (1 << 2);            // syscall_dap
        const unused      = (1 << 3);            // not used
        const semaphores  = (1 << 4);            // semaphores
        const ep0         = (1 << 5);            // ep0
        const ep1         = (1 << 6);            // ep1
        const ddft        = (1 << 7);            // ddft
        const struct8     = (1 << 8);
        const struct9     = (1 << 9);
        const struct10    = (1 << 10);
        const struct11    = (1 << 11);
        const struct12    = (1 << 12);
        const struct13    = (1 << 13);
        const struct14    = (1 << 14);
        const struct15    = (1 << 15);
        const none        = (1 >> 1);                //the zero option.
    }
}
/// ChannelConfig holds configuration masks that enable the configuration of the
/// path of release/notify events to an interrupt.
/// The release_mask and notify_mask associate the IPC channel with zero or more
/// Interrupt structures.
/// The intr_release_mask adn intr_notify_mask associate the Interrupt structure
/// with zero or more CPUSS_IPC interrupts.
/// --release_mask: release events sent to all intr_struct with set bits.                  
/// --notify_mask: notify events sent to all intr_struct with set bits.                   
/// --intr_release_mask: release event from configured channel triggers interrupt with set bits
/// --intr_notify_mask: notify event from configured channel triggers interrupt with set bits
/// Configure the channel notify and release masks to use zero or more particular Interrupt structure
/// Configure the Interrupt structure release and notify masks to use zero or more particular interrupt.
#[derive(Debug)]
pub struct ChannelConfig {
    pub struct_release: IntrStructMaskBits, // release events sent to all intr_struct with set bits.
    pub struct_notify: IntrStructMaskBits,  // notify events sent to all intr_struct with set bits.
    pub intr_release_mask: InterruptMaskBits, // release event from configured channel triggers interrupt with set bits
    pub intr_notify_mask: InterruptMaskBits, // notify event from configured channel triggers interrupt with set bits
}

/// Callback functions are called and executed when a channel notify or release event is
/// received.

macro_rules! ipc{
    ([
        $($C:ident: ($c:ident, $structi:ident,  $LOCK:ty)),+
    ], [
        $($IS:ident: ($is:ident, $intr_structi:ident,  $INTR_LOCK:ty)),+

    ]) => {

        //Create the channels
        //#[derive(Debug)]
        pub struct Channels{
            $(
                //Channel
                pub $c: $C<$LOCK>,
            )+
        }
        //Create the IntrStructs
        //#[derive(Debug)]
        pub struct IntrStructs{
            $(
                //IntrStruct
                pub $is: $IS<$INTR_LOCK>,
            )+
        }
        impl IpcChannel for IPC {
            type Channels = Channels;
            type IntrStructs = IntrStructs;
            fn split(self) -> (Channels, IntrStructs) {
                (Channels {
                    $(
                        $c: $C { _lock: PhantomData},
                    )+
                },
                IntrStructs{
                    $(
                        $is: $IS{ _intr_lock: PhantomData},
                        )+
                })
            }
        }
        $(
            ///Channel
            //#[derive(Debug)]
            pub struct $C<LOCK> {
                _lock: PhantomData<LOCK>,
            }
            impl<LOCK> $C<LOCK>{
                ///Configures the channel to start released.
                pub fn into_released(self)->$C<Released>{
                    $C{ _lock: PhantomData }
                }
                // release_lock releases the channel lock and causes a release
                // interrupt for the bits set in the release_intr_mask.
                // If no interrupt is required clear all bits in release_intr_mask.
                // #Safety - Single instruction write.
                #[inline]
                pub fn release_lock(&mut self, release_intr_mask: &IntrStructMaskBits) ->Result<$C<Released>, Error> {
                    unsafe{(*IPC::PTR)
                                 .$structi
                                 .release
                                 .write(|w|
                                        w.intr_release()
                                        .bits(release_intr_mask.bits() as u16))}
                    Ok( $C{ _lock: PhantomData::<Released>})
                }
                // notify sends a notification to all the IPC Interrupt structures
                // that have a set bit in notify_intr_mask.
                 #[inline(always)]
                pub fn notify(&self, notify_intr_mask: &IntrStructMaskBits) ->() {
                    unsafe{(*IPC::PTR)
                           .$structi
                           .notify
                           .write(|w|
                                  w.intr_notify()
                                  .bits(notify_intr_mask.bits() as u16))
                    }
                }
                //is_locked returns the lock status of the ipc channel.
                //#Safety: Deref of symbolic pointer, single instruction 
                //         read of register with no side effects.
                #[inline(always)]
                pub fn is_locked(&self)-> bool{
                    unsafe{
                        (*IPC::PTR)
                            .$structi
                            .lock_status
                            .read()
                            .acquired()
                            .bit_is_set()
                    }
                }
                // read_data_register reads and returns the byte of
                // data in the IPC channel data register.
                // Do not use this method directly use one of the HAL
                // IPC Channel methods.
                // Safety: Single instruction read.
                #[inline(always)]
                pub(crate) fn read_data_register(&self) -> u32{
                    unsafe{
                        (*IPC::PTR)
                            .$structi
                            .data
                            .read()
                            .bits()
                    }
                }
            }
            impl $C<Released>{


                /// acquire_lock attempts to acquire an IPC channel lock.
                /// When acquired a Channel is returned that can be used to
                /// send data.
                /// acquire_lock will try upto 200 times to acquire the lock.
                /// If a lock cannot be acquired an Error::ChannelBusy is returned.
                #[inline]
                pub(crate) fn acquire_lock(&mut self) ->Result<$C<Acquired>, Error> {
                    let mut count:u8 = 0;
                    //#safety: single instruction read of register.
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
                        Err(Error::ChannelBusy)
                    }
                }
            }

            impl $C<Acquired>{
                // write_data_register writes directly to the IPC channel data register.
                // Don't use this method directly use the HAL IPC functionality.
                // byte of data between channels.
                // Input args are:
                // - a byte of data.
                // () is returned.
                // unsafe because no precondition testing to ensure synchornised access
                // to data register.
                #[inline(always)]
                pub(crate) unsafe fn write_data_register(&self, data: u32) -> (){
                    unsafe{
                        (*IPC::PTR)
                            .$structi
                            .data
                            .write(|w| w.bits(data));
                    }
                }
               
            }

            /// Intr_struct
            #[allow(non_camel_case_types)]
            ////#[derive(Debug)]
            pub struct $IS<INTR_LOCK> {
                _intr_lock: PhantomData<INTR_LOCK>,
            }
            impl<LOCK> $IS<LOCK>{
                pub fn new()-> $IS<Released>{
                    $IS{_intr_lock: PhantomData }
                }
                // #[allow(unused_variables)]
                // fn release_lock(&mut self, release_intr_mask: &IntrStructMaskBits) -> Result<$IS<Released>, Error>{
                //     todo!("Implement lock for intr_struct config.")
                // }
                // fn acquire_lock() -> Result<$IS<Acquired>, Error>{
                //     todo!("Implement lock for intr_struct config.")
                // }
                /// configure_channel configures the channel release and notify channels,
                /// and the interrupt structure interrupts and mask.
                // #[inline]
                // pub fn configure(&mut self,
                //                  release_interrupt: &InterruptMaskBits,
                //                  notify_interrupt: &InterruptMaskBits,
                //                  _cs:&CriticalSection)-> Result<(), Error>{
                //     //#Safety: Safe within critical section.
                //     unsafe{
                //         self.configure_release(release_interrupt);
                //         self.configure_notify(notify_interrupt);
                //     }

                //     Ok(())
                // }
                //configure_channel_notify_intr_structure associates
                //one or more interrupt structures with the IPC
                //channel.
                //A notification will be sent to every structure with
                //a set bit.
                //#Safety: Critical section avoids data race. Dereferenced pointer
                //         
                 #[inline(always)]
                pub fn configure_notify(&mut self, intr_struct_mask: &InterruptMaskBits, _cs:&CriticalSection) ->() {
                    unsafe{(*IPC::PTR)
                           .$intr_structi
                           .intr_mask
                           .write(|w|
                                  w.notify()
                                  .bits(intr_struct_mask.bits() as u16))
                    }
                }
                // configure_channel_release_intr_structure works similarly to
                // configure_channel_notify_intr_structure.
                //#Safety: Unsafe due to no synchronisation on register.
                //         Potential data race  when called outside of system startup.
                 #[inline(always)]
                pub fn configure_release(&mut self, intr_struct_mask: &InterruptMaskBits, _cs:&CriticalSection) ->() {
                    unsafe{(*IPC::PTR)
                           .$intr_structi
                           .intr_mask
                           .write(|w|
                                  w.release()
                                  .bits(intr_struct_mask.bits() as u16))
                    }
                }
                // clear_release_interrupts clears any release
                // interrupts set for the associated IPC channel
                // interrupt structure.
                //#Safety: Unsafe due to no synchronisation on register.
                //         Potential data race when called outside of system startup.
                #[inline(always)]
                unsafe fn clear_release_interrupts(&mut self, release_mask: InterruptMaskBits)->(){
                    unsafe{(*IPC::PTR)
                           .$intr_structi
                           .intr
                           .write(|w|
                                  w.release()
                                  .bits(release_mask.bits() as u16))
                    }
                }
                /// clear_notify_interrupts clears any notification interrupts set for the
                /// associated IPC channel interrupt structure.
                //#Safety: Unsafe due to no synchronisation on register.
                //         Potential data race
                #[inline(always)]
                unsafe fn clear_notify_interrupts(&mut self, notify_mask: InterruptMaskBits)->(){
                    unsafe{(*IPC::PTR)
                           .$intr_structi
                           .intr
                           .write(|w|
                                  w.notify()
                                  .bits(notify_mask.bits() as u16))
                    }
                }
            }

        )+

    };
}
/*
IpcChannel structs are created by the lines in the ipc macro.
The lines represent the software abstraction of the IPC channel and its
associated hardware registers.
e.g. SyscallCm0: (syscall_cm0, struct0, Released),
 --SyscallCm0 is the software IPC channel type
 --syscall_cm0 -- the software struct representing the IPC Channel
 --struct0 -- the hardware IPC channel registers.
 --intr_struct0 -- the hardware interrupt structure associated with the
                   IPC channel hardware registers.
 --Released -- the initial state of the software IPC channel abstraction
               when the software IPC channel is created.
*/
ipc!(
    [
        SyscallCm0: (syscall_cm0, struct0, Released),
        SyscallCm4: (syscall_cm4, struct1, Released),
        SyscallDap: (syscall_dap, struct2, Released),
        Reserved: (reserved, struct3, Released),
        Semaphores: (semaphores, struct4, Released),
        PipeEp0: (pipe_ep0, struct5, Released),
        PipeEp1: (pipe_ep1, struct6, Released),
        Ddft: (ddft, struct7, Released),
        Channel8: (channel8, struct8, Released),
        Channel9: (channel9, struct9, Released),
        Channel10: (channel10, struct10, Released),
        Channel11: (channel11, struct11, Released),
        Channel12: (channel12, struct12, Released),
        Channel13: (channel13, struct13, Released),
        Channel14: (channel14, struct14, Released),
        Channel15: (channel15, struct15, Released)
    ],
    [
        Syscall:      (syscall,       intr_struct0,  Released),
        IntrStruct1:  (intr_struct1,  intr_struct1,  Released),
        IntrStruct2:  (intr_struct2,  intr_struct2,  Released),
        IntrEp0:      (intr_ep0,      intr_struct3,  Released),
        IntrEp1:      (intr_ep1,      intr_struct4,  Released),
        IntrStruct5:  (intr_struct5,  intr_struct5,  Released),
        IntrStruct6:  (intr_struct6,  intr_struct6,  Released),
        IntrSpare:    (intr_spare,    intr_struct7,  Released),
        IntrStruct8:  (intr_struct8,  intr_struct8,  Released),
        IntrStruct9:  (intr_struct9,  intr_struct9,  Released),
        IntrStruct10: (intr_struct10, intr_struct10, Released),
        IntrStruct11: (intr_struct11, intr_struct11, Released),
        IntrStruct12: (intr_struct12, intr_struct12, Released),
        IntrStruct13: (intr_struct13, intr_struct13, Released),
        IntrStruct14: (intr_struct14, intr_struct14, Released),
        IntrStruct15: (intr_struct15, intr_struct15, Released)
    ]
);
