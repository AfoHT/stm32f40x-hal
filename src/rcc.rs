//! Reset and Clock Control

use core::cmp;

use cast::u32;
use stm32f40x::{rcc, RCC};

use flash::ACR;
use time::Hertz;

/// Extension trait for the 'RCC' peripheral
pub trait RccExt {
    /// Constrain the 'RCC' peripheral, preventing direct access
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            ahb1: AHB1 { _0: () },
            ahb2: AHB2 { _0: () },
            ahb3: AHB3 { _0: () },
            apb1: APB1 { _0: () },
            apb2: APB2 { _0: () },
            cfgr: CFGRBuilder {
                source: ClockSource::HSI,
                pll: None,
                ahb_prescale: None,
                apb1_prescale: None,
                apb2_prescale: None,
            },
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    pub ahb1: AHB1,
    pub ahb2: AHB2,
    pub ahb3: AHB3,
    pub apb1: APB1,
    pub apb2: APB2,
    pub cfgr: CFGRBuilder,
}

/// AMBA High-performance Bus 1 (AHB1) register
pub struct AHB1 {
    _0: (),
}

impl AHB1 {
    /// Enable register
    pub(crate) fn enr(&mut self) -> &rcc::AHB1ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb1enr }
    }

    /// Low power enable register
    pub(crate) fn lpenr(&mut self) -> &rcc::AHB1LPENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb1lpenr }
    }

    /// Reset register
    pub(crate) fn rstr(&mut self) -> &rcc::AHB1RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb1rstr }
    }
}

/// AMBA High-performance Bus 2 (AHB2) register
pub struct AHB2 {
    _0: (),
}

impl AHB2 {
    /// Enable register
    pub(crate) fn enr(&mut self) -> &rcc::AHB2ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb2enr }
    }

    /// Low power enable register
    pub(crate) fn lpenr(&mut self) -> &rcc::AHB2LPENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb2lpenr }
    }

    /// Reset register
    pub(crate) fn rstr(&mut self) -> &rcc::AHB2RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb2rstr }
    }
}

/// AMBA High-performance Bus 3 (AHB3) register
pub struct AHB3 {
    _0: (),
}

impl AHB3 {
    /// Enable register
    pub(crate) fn enr(&mut self) -> &rcc::AHB3ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb3enr }
    }

    /// Low power enable register
    pub(crate) fn lpenr(&mut self) -> &rcc::AHB3LPENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb3lpenr }
    }

    /// Reset register
    pub(crate) fn rstr(&mut self) -> &rcc::AHB3RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).ahb3rstr }
    }
}

/// Advanced Peripheral Bus 1 (APB1) registers
pub struct APB1 {
    _0: (),
}

impl APB1 {
    /// Enable register
    pub(crate) fn enr(&mut self) -> &rcc::APB1ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1enr }
    }

    /// Low power enable register
    pub(crate) fn lpenr(&mut self) -> &rcc::APB1LPENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1lpenr }
    }

    /// Reset register
    pub(crate) fn rstr(&mut self) -> &rcc::APB1RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb1rstr }
    }
}

/// Advanced Peripheral Bus 2 (APB2) registers
pub struct APB2 {
    _0: (),
}

impl APB2 {
    /// Enable register
    pub(crate) fn enr(&mut self) -> &rcc::APB2ENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2enr }
    }

    /// Low power enable register
    pub(crate) fn lpenr(&mut self) -> &rcc::APB2LPENR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2lpenr }
    }

    /// Reset register
    pub(crate) fn rstr(&mut self) -> &rcc::APB2RSTR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*RCC::ptr()).apb2rstr }
    }
}

/// Clock source to use. HSI is the internal low-precision source at 16 MHz. HSE is an external
/// clock source between 4-26 MHz fed into OSC_IN.
pub enum ClockSource {
    HSI,
    HSE(Hertz),
}

/// Clock configuration register
pub struct CFGRBuilder {
    /// Clock source for system or pll. Defaults to HSI
    source: ClockSource,
    /// Pll clock. m, n, p, q coefficients
    pll: Option<(u32, u32, u32, u32)>,
    /// AHB bus clock prescaler
    ahb_prescale: Option<u32>,
    /// APB1 bus clock
    apb1_prescale: Option<u32>,
    /// APB2 bus clock
    apb2_prescale: Option<u32>,
}

impl CFGRBuilder {
    /// Sets the input clock source to be used for either the System clock directly or the PLL.
    /// Defaults to HSI
    pub fn source(mut self, source: ClockSource) -> Self {
        self.source = source;
        self
    }

    /// PLL enable flag. Takes in coefficients n, p and q
    pub fn enable_pll(mut self, pll_n: u32, pll_p: u32, pll_q: u32) -> Self {
        let pll_m = match self.source {
            ClockSource::HSI => 8,
            ClockSource::HSE(pll_input_freq) => (pll_input_freq.0 + 1_999_999) / 2_000_000,
        };

        self.pll = Some((pll_m, pll_n, pll_p, pll_q));

        self
    }

    /// Configures the AHB prescale value
    pub fn ahb_prescale(mut self, scale: u32) -> Self {
        self.ahb_prescale = Some(scale);
        self
    }

    /// Configures a frequency for the APB1 bus
    pub fn apb1_prescale(mut self, scale: u32) -> Self {
        self.apb1_prescale = Some(scale);
        self
    }

    /// Configures a frequency for the APB2 bus
    pub fn apb2_prescale(mut self, scale: u32) -> Self {
        self.apb2_prescale = Some(scale);
        self
    }

    /// Freeze configuration and actually update the clock frequencies
    pub fn build(self, acr: &mut ACR) -> Clocks {
        let rcc = unsafe { &*RCC::ptr() };

        // Calculate final sysclk (core) freq
        let sysclk_freq = match self.pll {
            Some((pll_m, pll_n, pll_p, _pll_q)) => {
                let vco = match self.source {
                    ClockSource::HSI => 16_000_000 / pll_m,
                    ClockSource::HSE(freq) => freq.0 / pll_m,
                };

                (vco * pll_n) / pll_p
            }
            None => match self.source {
                ClockSource::HSI => 16_000_000,
                ClockSource::HSE(freq) => freq.0,
            },
        };

        // Set AHB divisor
        let hclk_freq = {
            let ahb_prescale = self.ahb_prescale.unwrap_or(1);

            let ahb_prescale_bits = match ahb_prescale {
                1 => 0b0000,
                2 => 0b1000,
                4 => 0b1001,
                8 => 0b1010,
                16 => 0b1011,
                64 => 0b1100,
                128 => 0b1101,
                256 => 0b1110,
                512 => 0b1111,
                _ => panic!("Invalid ahb_prescale value (HPRE)"),
            };

            let hclk_freq = sysclk_freq / ahb_prescale;

            // TODO: Ethernet
            // assert!(hclk_freq >= 25_000_000);

            // AHB Max speed is 168 MHz
            assert!(hclk_freq <= 168_000_000);

            rcc.cfgr
                .write(|w| unsafe { w.hpre().bits(ahb_prescale_bits) });

            hclk_freq
        };

        // Set APB1 divisor
        let (pclk1_freq, ppre1) = {
            let apb1_prescale = self.apb1_prescale.unwrap_or(1);

            let apb1_prescale_bits = match apb1_prescale {
                1 => 0b000,
                2 => 0b100,
                4 => 0b101,
                8 => 0b110,
                16 => 0b111,
                _ => panic!("Invalid apb1_prescale value (PPRE1)"),
            };

            let apb1_freq = sysclk_freq / apb1_prescale;

            // APB low speed clock must not exceed 42 MHz
            assert!(apb1_freq <= 42_000_000);

            rcc.cfgr
                .write(|w| unsafe { w.ppre1().bits(apb1_prescale_bits) });

            (apb1_freq, apb1_prescale as u8)
        };

        // Set APB2 divisor
        let (pclk2_freq, ppre2) = {
            let apb2_prescale = self.apb2_prescale.unwrap_or(1);

            let apb2_prescale_bits = match apb2_prescale {
                1 => 0b000,
                2 => 0b100,
                4 => 0b101,
                8 => 0b110,
                16 => 0b111,
                _ => panic!("Invalid apb2_prescale value (PPRE2)"),
            };

            let apb2_freq = sysclk_freq / apb2_prescale;

            // APB low speed clock must not exceed 84 MHz
            assert!(apb2_freq <= 84_000_000);

            rcc.cfgr
                .write(|w| unsafe { w.ppre1().bits(apb2_prescale_bits) });

            (apb2_freq, apb2_prescale as u8)
        };

        // Adjust flash wait state
        acr.acr().write(|w| {
            if hclk_freq <= 30_000_000 {
                w.latency().bits(0b000)
            } else if hclk_freq <= 60_000_000 {
                w.latency().bits(0b001)
            } else if hclk_freq <= 90_000_000 {
                w.latency().bits(0b010)
            } else if hclk_freq <= 120_000_000 {
                w.latency().bits(0b011)
            } else if hclk_freq <= 150_000_000 {
                w.latency().bits(0b100)
            } else {
                // hclk_freq <= 168_000_000
                w.latency().bits(0b101)
            }
        });

        // Set and enable system clock source
        if let Some((pll_m, pll_n, pll_p, pll_q)) = self.pll {
            // Configure PLL src
            match self.source {
                ClockSource::HSI => rcc.pllcfgr.write(|w| w.pllsrc().internal()),
                ClockSource::HSE(_) => rcc.pllcfgr.write(|w| w.pllsrc().external()),
            }

            // Calculate VCO
            let vco_freq = match self.source {
                ClockSource::HSI => (16_000_000 / pll_m) * pll_n,
                ClockSource::HSE(freq) => (freq.0 / pll_m) * pll_n,
            };

            // Validate pll_m, pll_n, pll_p, pll_q
            assert!(pll_m >= 2 && pll_m <= 63);
            assert!(pll_n >= 50 && pll_n <= 432);
            assert!(pll_p == 2 || pll_p == 4 || pll_p == 6 || pll_p == 8);
            assert!(pll_q >= 2 && pll_q <= 15);

            assert!(vco_freq >= 100_000_000 && vco_freq <= 432_000_000);

            // Convert pll_p to bits
            let pll_p_bits = match pll_p {
                2 => 0b00,
                4 => 0b01,
                6 => 0b10,
                8 => 0b11,
                _ => panic!("Invalid pll_p value (PLLP)"),
            };

            // Set pll coefficients
            rcc.pllcfgr.write(|w| unsafe { w.pllm().bits(pll_m as u8) });
            rcc.pllcfgr
                .write(|w| unsafe { w.plln().bits(pll_n as u16) });
            rcc.pllcfgr.write(|w| unsafe { w.pllp().bits(pll_p_bits) });
            rcc.pllcfgr.write(|w| unsafe { w.pllq().bits(pll_q as u8) });

            // Set PLL as clock source
            rcc.cfgr.write(|w| w.sw().pll());
        } else {
            // Set either HSI or HSE as clock source
            match self.source {
                ClockSource::HSI => rcc.cfgr.write(|w| w.sw().hsi()),
                ClockSource::HSE(_) => rcc.cfgr.write(|w| w.sw().hse()),
            }
        }

        Clocks {
            hclk: Hertz(hclk_freq),
            pclk1: Hertz(pclk1_freq),
            pclk2: Hertz(pclk2_freq),
            ppre1,
            ppre2,
            sysclk: Hertz(sysclk_freq),
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration cannot be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    /// HCLK to AHB, core, memory and DMA
    hclk: Hertz,
    /// APB1 peripheral bus clock
    pclk1: Hertz,
    /// APB2 peripheral bus clock
    pclk2: Hertz,
    /// APB1 prescale constant
    ppre1: u8,
    /// APB2 prescale constant
    ppre2: u8,
    /// System (core) frequency
    sysclk: Hertz,
}

impl Clocks {
    /// Returns the frequency of the AHB
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns the frequency of the APB1
    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    /// Returns the frequency of the APB2
    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    pub(crate) fn ppre1(&self) -> u8 {
        self.ppre1
    }

    pub(crate) fn ppre2(&self) -> u8 {
        self.ppre2
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }
}
