//! scratch  example for testing ipc code
//! on the CY8CPROTO-063-BLE

#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]

use crate::psoc6_hal::drivers::ipc::IpcChannel;
//extern crate panic_semihosting;
extern crate psoc6_hal;

use cortex_m_rt::entry;
//use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let p = psoc6_pac::Peripherals::take().unwrap();

    let ipc = p.IPC.split();

    let _syscall0 = ipc.syscall_cm0;

    loop {}
}
