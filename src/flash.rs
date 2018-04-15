//! Flash memory

use stm32f40x::{flash, FLASH};

/// Extension trait to constraint the FLASH peripheral
pub trait FlashExt {
    /// Constrains the FLASH peripheral to prevent raw access
    fn constrain(self) -> Parts;
}

impl FlashExt for FLASH {
    fn constrain(self) -> Parts {
        Parts {
            acr: ACR { _0: () },
        }
    }
}

/// Constrained FLASH peripheral
pub struct Parts {
    // Opaque ACR register
    pub acr: ACR,
}

/// Opaque access control register (ACR)
pub struct ACR {
    _0: (),
}

impl ACR {
    pub(crate) fn acr(&mut self) -> &flash::ACR {
        // NOTE(unsafe) this proxy grantes exclusive access to this register
        unsafe { &(*FLASH::ptr()).acr }
    }
}

