use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::api::IrqIf;

pub struct MockIrqController {
    enabled: AtomicBool,
    last_irq: AtomicU32,
}

impl MockIrqController {
    pub const fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            last_irq: AtomicU32::new(0),
        }
    }

    pub fn was_enabled(&self) -> bool {
        self.enabled.load(Ordering::Acquire)
    }

    pub fn last_irq(&self) -> Option<u32> {
        if self.was_enabled() {
            Some(self.last_irq.load(Ordering::Relaxed))
        } else {
            None
        }
    }

    #[cfg(test)]
    pub fn reset(&self) {
        self.enabled.store(false, Ordering::Release);
        self.last_irq.store(0, Ordering::Release);
    }
}

impl IrqIf for MockIrqController {
    fn enable(&self, irq: u32) {
        self.last_irq.store(irq, Ordering::Release);
        self.enabled.store(true, Ordering::Release);
    }

    fn acknowledge(&self, irq: u32) {
        self.last_irq.store(irq, Ordering::Release);
        let _ = irq;
    }
}

pub static PLATFORM_IRQ: MockIrqController = MockIrqController::new();

pub fn platform_irq() -> &'static dyn IrqIf {
    &PLATFORM_IRQ
}

#[cfg(test)]
pub fn reset() {
    PLATFORM_IRQ.reset();
}
