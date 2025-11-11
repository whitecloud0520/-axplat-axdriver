#![cfg_attr(not(test), no_std)]

pub mod api;
pub mod driver;
pub mod mock;
#[cfg(not(test))]
pub mod platform;

#[cfg(test)]
pub mod platform {
    pub use crate::mock::platform::*;
}

pub use api::{ConsoleIf, DmaRegion, InitIf, IrqIf, MemIf};
pub use driver::{DevError, DevResult, DeviceType, DriverSummary};
pub use platform::AxUnifiedPlatform;
