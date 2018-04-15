//! HAL for the STM32F40x family of microcontrollers
//!
//! An implementation of the ['embedded-hal'] traits for the STM32F40x family of
//! microcontrollers.
//! 
//! ['embedded-hal']: https://github.com/japaric/embedded-hal
//! 
//! See ['stm32f30x-hal'], ['cortex-m-quickstart'] and ['f3'] for more understanding how to use the hal
//! 
//! ['stm32f30x-hal']: https://github.com/japaric/stm32f30x-hal
//! ['cortex-m-quickstart']: https://github.com/japaric/cortex-m-quickstart
//! ['f3']: https://github.com/japaric/f3

#![no_std]

extern crate cast;
extern crate cortex_m;
extern crate embedded_hal as hal;
extern crate nb;

pub extern crate stm32f40x;

pub mod rcc;
pub mod flash;
pub mod gpio;
pub mod time;
pub mod prelude;
pub mod delay;