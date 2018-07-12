//! HAL for the STM32F40x family of microcontrollers
//!
//! An implementation of the ['embedded-hal'] traits for the STM32F40x family of
//! microcontrollers.
//!
//! [`embedded-hal`]: https://github.com/japaric/embedded-hal
//!
//! # Requirements
//!
//! This crate requires `arm-none-eabi-gcc` to be installed and available in `$PATH` to build.
//!
//! # Usage
//!
//! To build applications (binary crates) using this crate follow the [cortex-m-quickstart]
//! instructions and add this crate as a dependency in step number 5 and make sure you enable the
//! "rt" Cargo feature of this crate.
//!
//! [cortex-m-quickstart]: https://docs.rs/cortex-m-quickstart/~0.3
//!
//! # Examples
//!
//! Examples of *using* these abstractions can be found in the documentation of the [`f3`] crate.
//!
//! [`f3`]: https://docs.rs/f3/~0.6

//#![deny(missing_docs)]
//#![deny(warnings)]
#![feature(never_type)]
#![no_std]

extern crate cast;
extern crate cortex_m;
extern crate embedded_hal as hal;
extern crate nb;
extern crate void;

pub extern crate stm32f40x;

pub mod delay;
pub mod flash;
pub mod gpio;

// Needs fixing!
//pub mod i2c;
pub mod prelude;
pub mod rcc;
pub mod serial;
pub mod spi;
pub mod time;
pub mod timer;
