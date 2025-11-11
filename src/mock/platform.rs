use core::sync::atomic::{AtomicBool, Ordering};

use heapless::Vec;
use spin::{Mutex, Once};

use crate::{
    api::InitIf,
    driver::{self, BaseDriverOps, DevResult, DriverRegistry, DriverSummary},
    mock,
};

static PLATFORM_STATE: Once<UnifiedPlatformState> = Once::new();

pub struct AxUnifiedPlatform;

struct UnifiedPlatformState {
    drivers: Mutex<DriverRegistry<'static>>,
    initialized: AtomicBool,
}

impl UnifiedPlatformState {
    const fn new() -> Self {
        Self {
            drivers: Mutex::new(DriverRegistry::new()),
            initialized: AtomicBool::new(false),
        }
    }

    fn ensure_initialized(&self) -> DevResult {
        if self.initialized.load(Ordering::Acquire) {
            return Ok(());
        }

        driver::prepare_builtin_drivers()?;

        let mut table = self.drivers.lock();
        for driver in driver::builtin_drivers() {
            table.register_driver(driver)?;
        }

        self.initialized.store(true, Ordering::Release);
        Ok(())
    }

    fn registry(&self) -> spin::MutexGuard<'_, DriverRegistry<'static>> {
        self.drivers.lock()
    }
}

impl AxUnifiedPlatform {
    fn state() -> &'static UnifiedPlatformState {
        PLATFORM_STATE.call_once(UnifiedPlatformState::new)
    }

    pub fn driver_summaries() -> Vec<DriverSummary, 8> {
        let state = Self::state();
        let _ = state.ensure_initialized();
        state.registry().summaries()
    }

    #[cfg(test)]
    pub fn reset_for_tests() {
        let state = Self::state();
        state.initialized.store(false, Ordering::Release);
        {
            let mut table = state.registry();
            table.clear();
        }
        driver::reset_for_tests();
        mock::console::reset();
        mock::irq::reset();
        mock::memory::reset();
        mock::log::reset();
    }
}

impl InitIf for AxUnifiedPlatform {
    fn init_early(cpu_id: usize, arg: usize) {
        mock::log::record(mock::log::Stage::PrimaryEarly, cpu_id, arg);
    }

    fn init_later(cpu_id: usize, arg: usize) {
        let state = Self::state();
        if let Err(err) = state.ensure_initialized() {
            mock::log::record_failure(cpu_id, err);
            return;
        }
        mock::log::record(mock::log::Stage::PrimaryLate, cpu_id, arg);
    }

    fn init_early_secondary(cpu_id: usize) {
        mock::log::record(mock::log::Stage::SecondaryEarly, cpu_id, 0);
    }

    fn init_later_secondary(cpu_id: usize) {
        mock::log::record(mock::log::Stage::SecondaryLate, cpu_id, 0);
    }
}

pub fn with_registered<F, R>(f: F) -> R
where
    F: FnOnce(&[&'static dyn BaseDriverOps]) -> R,
{
    let state = AxUnifiedPlatform::state();
    f(state.registry().entries())
}
