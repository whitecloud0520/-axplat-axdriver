use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DmaRegion {
    pub paddr: usize,
    pub size: usize,
    pub align: usize,
}

impl DmaRegion {
    pub const fn new(paddr: usize, size: usize, align: usize) -> Self {
        Self { paddr, size, align }
    }
}

pub trait InitIf: Send + Sync {
    fn init_early(cpu_id: usize, arg: usize);
    fn init_later(cpu_id: usize, arg: usize);
    fn init_early_secondary(cpu_id: usize);
    fn init_later_secondary(cpu_id: usize);
}

pub trait ConsoleIf: Send + Sync {
    fn write_bytes(&self, bytes: &[u8]);
    fn read_bytes(&self, buf: &mut [u8]) -> usize;
    fn supports_interrupts(&self) -> bool;
}

pub trait IrqIf: Send + Sync {
    fn enable(&self, irq: u32);
    fn acknowledge(&self, irq: u32);
}

pub trait MemIf: Send + Sync {
    fn dma_alloc(&self, bytes: usize, align: usize) -> Option<DmaRegion>;
    fn dma_flush(&self, region: &DmaRegion);
}

impl fmt::Debug for dyn ConsoleIf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConsoleIf(..)")
    }
}

impl fmt::Debug for dyn IrqIf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IrqIf(..)")
    }
}

impl fmt::Debug for dyn MemIf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MemIf(..)")
    }
}
