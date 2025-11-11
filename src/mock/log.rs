use crate::driver::DevError;
use heapless::Vec;
use spin::Mutex;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage {
    PrimaryEarly,
    PrimaryLate,
    SecondaryEarly,
    SecondaryLate,
    Failure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LogEvent {
    pub stage: Stage,
    pub cpu_id: usize,
    pub payload: usize,
}

static LOG_BUFFER: Mutex<Vec<LogEvent, 32>> = Mutex::new(Vec::new());

pub fn record(stage: Stage, cpu_id: usize, payload: usize) {
    let mut guard = LOG_BUFFER.lock();
    let _ = guard.push(LogEvent {
        stage,
        cpu_id,
        payload,
    });
}

pub fn record_failure(cpu_id: usize, err: DevError) {
    record(Stage::Failure, cpu_id, err as usize);
}

pub fn snapshot() -> Vec<LogEvent, 32> {
    LOG_BUFFER.lock().clone()
}

#[cfg(test)]
pub fn reset() {
    LOG_BUFFER.lock().clear();
}
