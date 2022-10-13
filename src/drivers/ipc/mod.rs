//! ipc/mod.rs implements inter-core communication using the
//! inter-process communication (IPC) registers.
//! Message passing between cores is implemented using the IPC registers
//! Essentially any messages can be shared but the systems using the
//! IPC are the BLE subsystem and the Flash subsystem
//!

use crate::pac::IPC;
use core::marker::PhantomData;

use bitflags::bitflags;
pub mod semaphore;

pub trait IpcChannel {
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
    ChannelBusy,
}

bitflags! {
    #[derive(Debug)]
    pub struct MaskBits:u32 {
        const struct0 = (1 << 0);            // syscall_cm0
        const struct1 = (1 << 1);            // syscall_cm4
        const struct2 = (1 << 2);            // syscall_dap
        const struct3 = (1 << 3);            // not used
        const struct4 = (1 << 4);            // semaphores
        const struct5 = (1 << 5);            // ep0
        const struct6 = (1 << 6);            // ep1
        const struct7 = (1 << 7);            // ddft
        const struct8 = (1 << 8);
        const struct9 = (1 << 9);
        const struct10 = (1 << 10);
        const struct11 = (1 << 11);
        const struct12 = (1 << 12);
        const struct13 = (1 << 13);
        const struct14 = (1 << 14);
        const struct15 = (1 << 15);
        const none = (1 >> 1);                //the zero option.
    }
}

#[derive(Debug)]
pub struct ChannelConfig {
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
                /// release_lock releases the channel lock and causes a release
                /// interrupt for the bits set in the release_intr_mask.
                /// If no interrupt is required clear all bits in release_intr_mask.
                 // #Safety - Single instruction read.
                #[inline]
                fn release_lock(self, release_intr_mask: MaskBits) ->Result<$C<Released>, Error> {
                    unsafe{(*IPC::PTR)
                                 .$structi
                                 .release
                                 .write(|w|
                                        w.intr_release()
                                        .bits(release_intr_mask.bits() as u16))}
                    Ok( $C{ _lock: PhantomData::<Released>})
                }
                /// notify sends a notification to all the IPC Interrupt structures
                /// that have a set bit in notify_intr_mask.
                 #[inline(always)]
                fn notify(&mut self, notify_intr_mask: MaskBits) ->() {
                    unsafe{(*IPC::PTR)
                           .$structi
                           .notify
                           .write(|w|
                                  w.intr_notify()
                                  .bits(notify_intr_mask.bits() as u16))
                    }
                }
            }
            impl $C<Released>{
                /// configure_release_interrupts configures the interrupt structure.
                /// When a release event occurs the configured interrupt structures
                /// are sent an event. When this interrupt structure receives and event
                /// all the set bits in the mask will trigger interrupts.
                /// e.g. when ipc_intr_struct3 recieves an event and
                /// has a mask of 0x0001 then the CPUSS_INTERRUPTS_IPC_0 will be triggered.
                /// Any ipc_channel can be configured to use any of the ipc_intr_structs
                /// So if IPC struct0 (IPC_channel0) configures its ipc_struct_intr
                /// with mask 0x0004 then the intr_struct3 will recieve an interrupt
                /// event which will then trigger the masked interrupts.
                ///
                /// The interrupts can also be managed directly by the intr_structs:
                /// - Read the INTR_MASKED register to find any active interrupts for
                ///   the interrupt structure.
                /// - To activate an interrupt, write to the SET register.
                /// - To clear an interrupt, write to the INTR register.
                #[inline(always)]
                pub fn configure_release_interrupts(& self, release_intr_mask: MaskBits)-> (){
                    // #safety: two writes as per IPC chapter 6 pp 40 in TRM.
                    unsafe{
                        (*IPC::PTR)
                            .$intr_structi
                            .intr_mask
                            .write(|w|
                                    w
                                    .release()
                                   .bits(release_intr_mask.bits()as u16));
                    }
                }
                /// configure_notify_interrupts behave similarly to configure_release_interrupts.
                #[inline(always)]                
                pub fn configure_notify_interrupts(& self, notify_intr_mask: MaskBits)-> (){
                    // #safety: two writes as per IPC chapter 6 pp 40 in TRM.
                    unsafe{
                        (*IPC::PTR)
                            .$intr_structi
                            .intr_mask
                            .write(|w|
                                    w
                                    .notify()
                                    .bits(notify_intr_mask.bits() as u16));
                    }

                }
                /// acquire_lock attempts to acquire an IPC channel lock.
                /// When acquired a Channel is returned that can be used to
                /// send data.
                /// acquire_lock will try upto 200 times to acquire the lock.
                /// If a lock cannot be acquired an Error::ChannelBusy is returned.
                #[inline]
                fn acquire_lock(self) ->Result<$C<Acquired>, Error> {
                    let mut count:u8 = 0;
                    //#safety: single byte read from register.
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
               
                
                // write_data_register writes directly to the IPC channel data register.
                // Don't use this method directly use the HAL IPC functionality.
                // byte of data between channels.
                // Input args are:
                // - a byte of data.
                // () is returned.
                // #Safety - Safe locked single instruction write to IPC PTR.
                #[inline(always)]
                pub(crate) fn write_data_register(self, data: u32, notify:MaskBits ) -> (){
                    unsafe{
                        (*IPC::PTR)
                            .$structi
                            .data
                            .write(|w| w.bits(data));
                    }
                }
                // read_data_register reads and returns the byte of
                // data in the IPC channel data register.
                // Do not use this method directly use one of the HAL
                // IPC Channel methods.
                // Safety: Deref of PAC pointer.
                #[inline(always)]
                pub(crate) fn receive_data_register(self, release: MaskBits) -> u32{
                    unsafe{
                        (*IPC::PTR)
                            .$structi
                            .data
                            .read()
                            .bits() 
                    }
                }

                //configure_channel_notify_intr_structure associates
                //one or more interrupt structures with the IPC
                //channel.
                //A notification will be sent to every structure with
                //a set bit.
                 #[inline(always)]
                fn configure_channel_notify_intr_structure(&mut self, intr_structure_mask: MaskBits) ->() {
                    unsafe{(*IPC::PTR)
                           .$intr_structi
                           .intr_mask
                           .write(|w|
                                  w.notify()
                                  .bits(intr_structure_mask.bits() as u16))
                    }
                }
                // configure_channel_release_intr_structure works similarly to
                // configure_channel_notify_intr_structure.
                 #[inline(always)]
                fn configure_channel_release_intr_structure(&mut self, intr_structure_mask: MaskBits) ->() {
                    unsafe{(*IPC::PTR)
                           .$intr_structi
                           .intr_mask
                           .write(|w|
                                  w.release()
                                  .bits(intr_structure_mask.bits() as u16))
                    }
                }
                // clear_release_interrupts clears any release
                // interrupts set for the associated IPC channel
                // interrupt structure.
                #[inline(always)]
                fn clear_release_interrupts(&mut self, release_mask: MaskBits)->(){
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
                #[inline(always)]
                fn clear_notify_interrupts(&mut self, notify_mask: MaskBits)->(){
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
// IpcChannel structs are created by the lines in the ipc macro.
// The lines represent the software abstraction of the IPC channel and its
// associated hardware registers. 
// e.g. SyscallCm0: (syscall_cm0, struct0, intr_struct0, Released),
//  --SyscallCm0 is the software IPC channel type 
//  --syscall_cm0 -- the software struct representing the IPC Channel
//  --struct0 -- the hardware IPC channel registers.
//  --intr_struct0 -- the hardware interrupt structure associated with the
//                    IPC channel hardware registers.
//  --Released -- the initial state of the software IPC channel abstraction
//                when the software IPC channel is created.
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
