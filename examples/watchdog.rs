//! Watchdog example for the CY8CPROTO-063-BLE that uses
//! driver level functions.
//! To run this example:
//! > cargo build --target=thumbv6m-none-eabi --example watchdog
//! open openocd in a terminal:
//! > cd <to psoc6_hal directory>
//! > openocd
//! Open another terminal and navigate to the psoc6_hal/thumbv6m-none-eabi/target/
//!   <debug or release directory>/examples directory
//! > arm-none-eabi-gdb -q watchdog
//! (gdb) target extended-remote :3333
//! (gdb) load watchdog
//! The board is now programmed.
//! Stop openocd and gdb to see the LEDs indicate the watchdog resets.
//! Note the WDT reset will not be triggered while the debugger and openocd are still connected to the board.


#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m::interrupt::free;

use psoc6_hal::delay::Delay;
use psoc6_hal::prelude::*;
use psoc6_hal::pac::Peripherals;
use psoc6_hal::drivers::system::System;
use psoc6_hal::drivers::system::reset_cause;
#[cfg(not(armv7m))]
#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let system = System::from(p.SRSS);
    system.wdt_start(6000u32);
    let gpio = p.GPIO.split();

    let (mut led_red, mut led_green) = free(|cs| {(
            gpio.p6_3.into_strong_output(cs),
            gpio.p7_1.into_strong_output(cs),
        )});

    let mut delay = Delay::new(cp.SYST);

    //When a reset is caused by the WDT only the led_green will be on
    // Pressing the reset button on the board will cause both the
    // led_green and led_red to be on for the first 4 secs.
    // Once the WDT has reset the device only the led_green should be on
    // after reset.
    let last_reset = system.last_reset(); //separated to simplify debugging.
    if last_reset == reset_cause::ResetCause::WDTReset {
        led_red.set_high().unwrap();
    } else {
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
        if count < 5 {
            system.wdt_feed();
        }
        count += 1;
    }
}
