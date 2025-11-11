use heapless::Vec;
use spin::Mutex;

use crate::api::ConsoleIf;

pub struct MockConsole {
    buffer: Mutex<Vec<u8, 256>>,
}

impl MockConsole {
    pub const fn new() -> Self {
        Self {
            buffer: Mutex::new(Vec::new()),
        }
    }

    pub fn snapshot(&self) -> Vec<u8, 256> {
        self.buffer.lock().clone()
    }

    pub fn clear(&self) {
        self.buffer.lock().clear();
    }
}

impl ConsoleIf for MockConsole {
    fn write_bytes(&self, bytes: &[u8]) {
        let mut guard = self.buffer.lock();
        for &b in bytes {
            if guard.push(b).is_err() {
                break;
            }
        }
    }

    fn read_bytes(&self, buf: &mut [u8]) -> usize {
        let mut guard = self.buffer.lock();
        let len = buf.len().min(guard.len());
        for (dst, src) in buf.iter_mut().zip(guard.iter().take(len)) {
            *dst = *src;
        }
        for _ in 0..len {
            let _ = guard.remove(0);
        }
        len
    }

    fn supports_interrupts(&self) -> bool {
        true
    }
}

pub static PLATFORM_CONSOLE: MockConsole = MockConsole::new();

pub fn platform_console() -> &'static dyn ConsoleIf {
    &PLATFORM_CONSOLE
}

#[cfg(test)]
pub fn reset() {
    PLATFORM_CONSOLE.clear();
}
