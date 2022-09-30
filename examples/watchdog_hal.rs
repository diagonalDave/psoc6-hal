//! Watchdog example for the CY8CPROTO-063-BLE

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate psoc6_hal;
use psoc6_hal::prelude::*;
use cortex_m_rt::entry;

use psoc6_hal::psoc ;
//! This file indicates the future state of the hal.
//! It doesn't work yet because of various ownership rules.
#[entry]
fn main() -> ! {
    let psoc = psoc::Psoc::new();
    let wdt = psoc.watchdog();

    wdt.start(6000.0);
    
    let gpio = psoc.GPIO();

    let mut led_red = gpio.p6_3.into_strong_output();
    let mut led_green = gpio.p7_1.into_strong_output();

    let mut delay = psoc.Delay();

    //on Reset led_green is on for 4 seconds. The led_red is off
    //when running only the led_red will blink.
    led_red.set_high();
    led_green.set_low();
    delay.delay_ms(4000u32);
    led_green.set_high();
    let mut count = 0;
    loop {
        led_red.set_low();
        delay.delay_ms(400u32);

        led_red.set_high();
        delay.delay_ms(400u32);
        if count < 5{
            wdt.feed();
        }
        count += 1;
    }
}
