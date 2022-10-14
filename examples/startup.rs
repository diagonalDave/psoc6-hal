//! startup.rs is an example to test the start up code.
//! If the code startsup successfully the leds start blinking.

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_semihosting;

use psoc6_hal::{delay::Delay, ipc::SystemChannel, prelude::*, psoc::Psoc};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let psoc = Psoc::new();
    psoc.start_cm0p();
    pub struct Data<'a> {
        pub ptr: u32,
        pub data: &'a str,
    }
    let dats = Data {
        ptr: 3452u32,
        data: "stuff",
    };
    psoc.ipc
        .send_data(/*ptr to dats*/ SystemChannel::SyscallCm0, &dats.data);
    let mut led3 = psoc.gpio.p6_3.into_strong_output();
    let mut led4 = psoc.gpio.p7_1.into_strong_output();

    let mut delay = Delay::new(cp.SYST);

    loop {
        led3.set_low().unwrap();
        led4.set_high().unwrap();
        delay.delay_ms(1000u32);

        led3.set_high().unwrap();
        led4.set_low().unwrap();
        delay.delay_ms(1000u32);
    }
}
