//! clocky.rs is an example that shows some of the basic clock functionality.
//! The example starts the psoc clocks then configures:
//! - a 1Hz that is used to flash the led_red.
//! - a 2Hz that is used to flash the led_green.

#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_semihosting;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;

use psoc6_hal::{
    clocks,
    pac::{interrupt, Peripherals},
    prelude::*,
};

enum LedState {
    On,
    Off,
}
static mut TOGGLE_RED: Mutex<LedState> = Mutex::new(LedState::Off);
static mut TOGGLE_GREEN: Mutex<LedState> = Mutex::new(LedState::Off);

#[allow(non_snake_case)]
#[interrupt]
fn IOSS_INTERRUPTS_GPIO_0() {
    //do something swift
    free(|cs| unsafe {
        let mut led_state = TOGGLE_RED.borrow(cs);
        match led_state {
            LedState::On => {
                led_state = &LedState::Off;
            }
            LedState::Off => {
                led_state = &LedState::On;
            }
        }
    });
}

#[allow(non_snake_case)]
#[interrupt]
fn IOSS_INTERRUPTS_GPIO_1() {
    //do something extra swift.
    free(|cs| unsafe {
        let mut led_state = TOGGLE_GREEN.borrow(cs);
        match led_state {
            LedState::On => {
                led_state = &LedState::Off;
            }
            LedState::Off => {
                led_state = &LedState::On;
            }
        }
    });
}
#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    clocks::start_system_clocks(&p);
    let gpio = p.GPIO.split();

    let mut led_red = gpio.p6_3.into_strong_output();
    let mut led_green = gpio.p7_1.into_strong_output();

    let _clock_1hz = clocks::new(1u32); // this will need to be a timer rather than a clock
    let _clock_2hz = clocks::new(2u32);
    free(|cs| unsafe {
        let led_state = TOGGLE_RED.borrow(cs);
        match led_state {
            LedState::On => led_red.set_low().unwrap(),
            LedState::Off => led_red.set_high().unwrap(),
        }
    });
    free(|cs| unsafe {
        let led_state = TOGGLE_GREEN.borrow(cs);
        match led_state {
            LedState::On => led_green.set_low().unwrap(),
            LedState::Off => led_green.set_high().unwrap(),
        }
    });
    loop {}
}
