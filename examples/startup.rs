//! startup.rs is an example to test the start up code.
//! If the code startsup successfully the leds start blinking.

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate cortex_m_rt;

use psoc6_hal::{
    delay::Delay,
    prelude::*,
    psoc::Psoc,
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {

    let cp = cortex_m::Peripherals::take().unwrap();
    let psoc = Psoc::new();
    psoc.start_cm0p();
    
        
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
