//! Blinky example for the CY8CPROTO-063-BLE

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate psoc6_hal;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use psoc6_hal::peripherals::{
    delay::Delay,
    gpio::GpioExt,
};
use psoc6_hal::prelude::*;

#[entry]
fn main() -> ! {
    let p = psoc6_pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let gpio = p.GPIO.split();
    let wdt = psoc6_hal::drivers::watchdog_drv::WatchDog::new(&p.SRSS);
    wdt.start(6000.0);

    let mut led_red = gpio.p6_3.into_strong_output();
    let mut led_green = gpio.p7_1.into_strong_output();

    let mut delay = Delay::new(cp.SYST);
    let mut count = 0;
    led_red.set_high().unwrap();
    led_green.set_low().unwrap();
    delay.delay_ms(4000u32);
    led_green.set_high().unwrap();
    
    loop {
        hprintln!("red led low").unwrap();
        led_red.set_low().unwrap();
        delay.delay_ms(400u32);

        hprintln!("red led high").unwrap();
        led_red.set_high().unwrap();
        delay.delay_ms(400u32);
        if count < 5{
            count += 1;
            wdt.feed();
        }else{
            //do nothing so the wdt triggers.
        }
        
        
    }
}
