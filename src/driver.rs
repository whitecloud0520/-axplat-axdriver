use core::sync::atomic::{AtomicBool, Ordering};

use crate::api::{ConsoleIf, IrqIf, MemIf};
use crate::mock;
use spin::Once;

pub type DevResult<T = ()> = Result<T, DevError>;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceType {
    Block,
    Char,
    Net,
    Display,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DevError {
    AlreadyExists,
    Io,
    NoMemory,
    Unsupported,
}

pub trait BaseDriverOps: Send + Sync {
    fn device_name(&self) -> &'static str;
    fn device_type(&self) -> DeviceType;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DriverSummary {
    pub name: &'static str,
    pub device_type: DeviceType,
}

pub struct DriverRegistry<'a> {
    list: heapless::Vec<&'a dyn BaseDriverOps, 8>,
}

impl<'a> DriverRegistry<'a> {
    pub const fn new() -> Self {
        Self {
            list: heapless::Vec::new(),
        }
    }

    pub fn register_driver(&mut self, driver: &'a dyn BaseDriverOps) -> DevResult {
        if self
            .list
            .iter()
            .any(|existing| existing.device_name() == driver.device_name())
        {
            return Err(DevError::AlreadyExists);
        }
        self.list.push(driver).map_err(|_| DevError::NoMemory)
    }

    pub fn entries(&self) -> &[&'a dyn BaseDriverOps] {
        self.list.as_slice()
    }

    pub fn summaries(&self) -> heapless::Vec<DriverSummary, 8> {
        let mut out = heapless::Vec::new();
        for drv in &self.list {
            let _ = out.push(DriverSummary {
                name: drv.device_name(),
                device_type: drv.device_type(),
            });
        }
        out
    }

    #[cfg(test)]
    pub fn clear(&mut self) {
        self.list.clear();
    }
}

pub struct ConsoleDriver {
    inner: &'static dyn ConsoleIf,
}

impl ConsoleDriver {
    pub const NAME: &'static str = "platform-console";
}

impl BaseDriverOps for ConsoleDriver {
    fn device_name(&self) -> &'static str {
        Self::NAME
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Char
    }
}

#[derive(Clone, Copy, Debug)]
pub enum VirtIoTransport {
    Mmio { base_paddr: usize, irq_line: u32 },
}

impl VirtIoTransport {
    pub const fn irq_line(&self) -> u32 {
        match *self {
            VirtIoTransport::Mmio { irq_line, .. } => irq_line,
        }
    }
}

pub struct VirtIoNetDriver {
    name: &'static str,
    transport: VirtIoTransport,
    irq: &'static dyn IrqIf,
    mem: &'static dyn MemIf,
    ready: AtomicBool,
}

impl VirtIoNetDriver {
    pub const NAME: &'static str = "virtio-net@10000000";

    pub fn initialize(&self) -> DevResult {
        if self.ready.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        let _buffer = self.mem.dma_alloc(4096, 4096).ok_or(DevError::NoMemory)?;
        self.irq.enable(self.transport.irq_line());
        Ok(())
    }

    #[cfg(test)]
    pub fn reset(&self) {
        self.ready.store(false, Ordering::Release);
    }
}

impl BaseDriverOps for VirtIoNetDriver {
    fn device_name(&self) -> &'static str {
        self.name
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Net
    }
}

fn console_driver() -> &'static ConsoleDriver {
    static INSTANCE: Once<ConsoleDriver> = Once::new();
    INSTANCE.call_once(|| ConsoleDriver {
        inner: mock::console::platform_console(),
    })
}

fn virtio_net_driver() -> &'static VirtIoNetDriver {
    static INSTANCE: Once<VirtIoNetDriver> = Once::new();
    INSTANCE.call_once(|| VirtIoNetDriver {
        name: VirtIoNetDriver::NAME,
        transport: VirtIoTransport::Mmio {
            base_paddr: 0x1000_0000,
            irq_line: 32,
        },
        irq: mock::irq::platform_irq(),
        mem: mock::memory::platform_memory(),
        ready: AtomicBool::new(false),
    })
}

pub fn builtin_drivers() -> [&'static dyn BaseDriverOps; 2] {
    [console_driver(), virtio_net_driver()]
}

pub fn prepare_builtin_drivers() -> DevResult {
    use crate::api::ConsoleIf;
    mock::console::platform_console().write_bytes(b"[console] ready\n");
    virtio_net_driver().initialize()?;
    Ok(())
}

#[cfg(test)]
pub fn reset_for_tests() {
    virtio_net_driver().reset();
}
