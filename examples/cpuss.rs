#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;

use psoc6_hal::peripherals::delay::Delay;
use psoc6_hal::prelude::*;
use psoc6_hal::drivers::cpuss::Cpuss;

#[entry]
fn main() -> ! {
    let p = psoc6_pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let gpio = p.GPIO.split();

    let mut led3 = gpio.p6_3.into_strong_output();
    let mut led4 = gpio.p7_1.into_strong_output();

    let mut delay = Delay::new(cp.SYST);
    /* Cpuss test methods*/
    let cpuss = Cpuss::from(p.CPUSS);
    cpuss.cores_cm4_init_clocks(0x01);
    cpuss.cores_cm0_init_clocks(0x01, 0x00);

    
    loop {
        led3.set_low().unwrap();
        led4.set_high().unwrap();
        delay.delay_ms(1000u32);

        led3.set_high().unwrap();
        led4.set_low().unwrap();
        delay.delay_ms(1000u32);
    }
}
