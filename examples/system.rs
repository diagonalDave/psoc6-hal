//! Watchdog example for the CY8CPROTO-063-BLE

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate psoc6_hal;

use cortex_m_rt::entry;

use psoc6_hal::peripherals::delay::Delay;
use psoc6_hal::drivers::system::{WatchDog, System}
use psoc6_hal::prelude::*;
//use psoc6_hal::watchdog2::WatchDog2; //{WatchDog, WatchDog2};

use psoc6_pac::Peripherals;


#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let system = System::from(p.SRSS);
    let wdt = WatchDog::from(system);

    wdt.start(6000.0);
    //wdt2.start(6000.0);
    let gpio = p.GPIO.split();

    let mut led_red = gpio.p6_3.into_strong_output();
    let mut led_green = gpio.p7_1.into_strong_output();

    let mut delay = Delay::new(cp.SYST);

    //on Reset led_green is on for 4 seconds. The led_red is off
    //when running only the led_red will blink.
    led_red.set_high().unwrap();
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
            wdt.feed();
        }
        count += 1;
    }
}
