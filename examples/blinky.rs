//! Blinky example for the CY8CPROTO-063-BLE
//! To run this example:
//! > cargo build --target=thumbv6m-none-eabi --example blinky
//! open openocd in a terminal:
//! > cd <to psoc6_hal directory>
//! > openocd
//! Open another terminal and navigate to the psoc6_hal/thumbv6m-none-eabi/target/
//!   <debug or release directory>/examples directory
//! > arm-none-eabi-gdb -q blinky
//! (gdb) target extended-remote :3333
//! (gdb) load blinky
//! The board is now programmed and you have two options:
//! 1. Stop openocd and gdb then reset the board to see the LEDs happily blinking.
//! 2. If you uncommented the hprintln! lines then continue in gdb with:
//! (gdb) monitor semi-hosting enable
//! (gdb) continue
//! this will display blinking leds and output to the openocd terminal.
//! Note: this example does not work without the monitor command above.


#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
extern crate panic_semihosting;
use cortex_m_rt::entry;
//use cortex_m_semihosting::hprintln; //uncomment this line if you want text output to semihosting.
use cortex_m::interrupt::free;

use psoc6_hal::{psoc::Psoc, delay::Delay};
use psoc6_hal::prelude::*;

#[entry]
fn main() -> ! {
    let psoc = Psoc::new();
    let cp = cortex_m::Peripherals::take().unwrap();

    let ( mut led3, mut led4) = free(|cs| {(
            psoc.gpio.p6_3.into_strong_output(cs),
            psoc.gpio.p7_1.into_strong_output(cs),
        )});


    let mut delay = Delay::new(cp.SYST);

    loop {
        
        //hprintln!("red led low").unwrap(); //uncomment this line if you want text output.
        led3.set_low().unwrap();
        led4.set_high().unwrap();
        delay.delay_ms(1000u32);

       // hprintln!("red led high").unwrap();//uncomment this line if you want text output.
        led3.set_high().unwrap();
        led4.set_low().unwrap();
        delay.delay_ms(1000u32);
    }
}
