//! ipc/mod.rs implements inter-core communication using the
//! inter-process communication (IPC) registers.


use core::marker::PhantomData;
pub mod macro_sample;
// pub enum SystemChannel{
//     SyscallCm0 = 0,
//     SyscallCm4 = 1,
//     SyscallDap = 2,
//     Semaphores = 4,
//     PipeEp0 = 5,
//     PipeEp1 = 6,
//     Ddft = 7,
//     Nac,                        // Not a channel
// }
// pub enum UserChannel{
//     Channel8 = 8,
//     Channel9 = 9,
//     Channel10 = 10,
//     Channel11 = 11,
//     Channel12 = 12,
//     Channel13 = 13,
//     Channel14 = 14,
//     Channel15 = 15
// }

pub trait IpcChannel{
    type Channels;
    fn split(self) -> Self::Channels;
}
pub struct Acquired;
pub struct Released;
pub trait Lock {}
impl Lock for Acquired {}
impl Lock for Released {}


// macro_rules! ipc{
//     ([
//         $($C:ident: ($c:ident, $structi:expr)),+
//     ]) => {
//        // use core::convert::Infallible;
//         use crate::pac::IPC;

//         //Create the channels
//         pub struct Channels{
//             $(
//                 //Channel
//                 pub $c: $C,
//             )+
//         }
//         impl IpcChannel for IPC {
//             type Channels = Channels;
//             fn split(self) -> Channels {
//                 Channels {
//                     $(
//                         $c: $C { _lock: PhantomData},
//                     )+
//                 }
//             }
//         }
//         $(
//             ///Channel
//             pub struct $C <Released> {
//                 _lock: PhantomData::<Released>
//             }
//             )+
  
//     };
// }

// ipc! ([
//     SyscallCm0: (syscall_cm0, struct0),
//     SyscallCm4: (syscall_cm4, struct1),
//     SyscallDap: (syscall_dap, struct2),
//     Semaphores: (semaphores, struct4),
//     PipeEp0: (pipe_ep0, struct5),
//     PipeEp1: (pipe_ep1, struct6),
//     Ddft: (ddft, struct7),
//     Channel8: (channel8, struct8),
//     Channel9: (channel9, struct9),
//     Channel10: (channel10, struct10),
//     Channel11: (channel11, struct11),
//     Channel12: (channel12, struct12),
//     Channel13: (channel13, struct13),
//     Channel14: (channel14, struct14),
//     Channel15: (channel15, struct15)
//     ]);
