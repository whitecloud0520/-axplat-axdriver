pub mod console;
pub mod irq;
pub mod log;
pub mod memory;
pub mod platform;

pub fn platform_console() -> &'static dyn crate::api::ConsoleIf {
    console::platform_console()
}

pub fn platform_irq() -> &'static dyn crate::api::IrqIf {
    irq::platform_irq()
}

pub fn platform_memory() -> &'static dyn crate::api::MemIf {
    memory::platform_memory()
}
