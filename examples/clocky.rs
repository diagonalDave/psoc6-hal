//! clocky.rs is an example that shows some of the basic clock functionality.
//! The example starts the psoc clocks then configures:
//! - a 1Hz that is used to flash the led_red.
//! - a 2Hz that is used to flash the led_green.



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
    psoc.start_system_clocks();
    //clocks::start_system_clocks(&p);
    //clocks::new(1u32);
    let _t_clock = psoc.create_clock(20_0000);
    psoc.create_cm0_clock();
        
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
