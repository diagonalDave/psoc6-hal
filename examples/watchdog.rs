//! Watchdog example for the CY8CPROTO-063-BLE

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate psoc6_hal;

use cortex_m_rt::entry;

use psoc6_hal::delay::Delay;
use psoc6_hal::prelude::*;
use psoc6_pac::Peripherals;
use psoc6_hal::drivers::system::{System, reset_cause};


#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let system = System::from(p.SRSS);
    system.wdt_start(6000u32);
    let gpio = p.GPIO.split();

    let mut led_red = gpio.p6_3.into_strong_output();
    let mut led_green = gpio.p7_1.into_strong_output();

    let mut delay = Delay::new(cp.SYST);

    //When a reset is caused by the WDT only the led_green will be on
    // Pressing the reset button on the board will cause both the
    // led_green and led_red to be on for the first 4 secs.
    // Once the WDT has reset the device only the led_green should be on
    // after reset.
    let last_reset = system.last_reset(); //separated to simplify debugging.
    if last_reset == reset_cause::ResetCause::WDTReset{
        led_red.set_high().unwrap();
    }else{
        led_red.set_low().unwrap();
    }
    led_green.set_low().unwrap();
    delay.delay_ms(4000u32);
    led_green.set_high().unwrap();
    let mut count = 0;
    loop {
        led_red.set_low().unwrap();
        delay.delay_ms(400u32);

        led_red.set_high().unwrap();
        delay.delay_ms(400u32);
        if count < 5{
            system.wdt_feed();
        }
        count += 1;
    }
}
