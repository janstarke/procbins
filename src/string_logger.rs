
use flexi_logger::*;
use flexi_logger::writers::*;
use std::{io::Write, ops::DerefMut, usize};
use std::sync::{Arc, Mutex};
pub struct StringLogger {
    formatter: FormatFunction,
    writer: Mutex<StringLogWriter>,
}

struct StringLogWriter {
    messages: Arc<Mutex<Vec<u8>>>,
}

impl StringLogger {
    pub fn new(messages: Arc<Mutex<Vec<u8>>>, formatter: FormatFunction) -> Self {
        Self {
            writer: Mutex::new(StringLogWriter::new(messages)),
            formatter,
        }
    }
}

impl StringLogWriter {
    pub fn new(messages: Arc<Mutex<Vec<u8>>>) -> Self {
        Self {messages}
    }
}

impl Write for StringLogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.messages.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl LogWriter for StringLogger {
    fn write(&self, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        let mut guard = self.writer.lock().unwrap();
        let writer = guard.deref_mut();
        (self.formatter)(writer, now, record)
    }

    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }
    fn max_log_level(&self) -> LevelFilter {
        LevelFilter::Info
    }
}