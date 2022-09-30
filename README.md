# `psoc6-hal`

[![crates.io](https://img.shields.io/crates/v/psoc6-hal.svg)](https://crates.io/crates/psoc6-hal)
[![docs.rs](https://docs.rs/psoc6-hal/badge.svg)](https://docs.rs/psoc6-hal/)
[![Build Status](https://travis-ci.com/psoc-rs/psoc6-hal.svg?branch=master)](https://travis-ci.com/psoc-rs/psoc6-hal)
[![license](https://img.shields.io/badge/license-apache%202.0-blue.svg)](https://opensource.org/licenses/apache-2.0)
[![license: mit](https://img.shields.io/badge/license-mit-yellow.svg)](https://opensource.org/licenses/mit)

> A Rust embedded-hal HAL for all MCUs in the Cypress PSoC6 family

This crate is currently a WIP.

## Intro
Currently the examples work but are only built for, and run on the
CM0plus core. As work progresses the CM4 core will be available with
FPU and thumb2 instructions. Further down the track the programmable
hardware will also be added to the mix.

## OpenOCD
OpenOCD can be used to program the psoc6 the Infineon fork is
recommended ([OpenOCD Infineon fork](https://github.com/Infineon/openocd) )
and you might as well download the manual ([Infineon OpenOCD user manual](https://www.infineon.com/dgdl/Infineon-Infineon_Programmer_2.1_OpenOCD_CLI_User_Guide-Software-v01_00-EN.pdf?fileId=8ac78c8c7e7124d1017e914972291587)).

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
