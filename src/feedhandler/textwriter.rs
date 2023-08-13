use file_rotate::{
    compression::Compression, suffix::AppendTimestamp, suffix::FileLimit, ContentLimit, FileRotate,
    TimeFrequency,
};
use std::io::Write;
use std::path::Path;

pub struct TextWriter {
    log: FileRotate<AppendTimestamp>,
}

impl TextWriter {
    pub fn new(path: &str) -> TextWriter {
        TextWriter {
            log: FileRotate::new(
                Path::new("tickerplant").join(path),
                AppendTimestamp::default(FileLimit::MaxFiles(14)),
                ContentLimit::Time(TimeFrequency::Daily),
                Compression::OnRotate(10),
                None,
            ),
        }
    }

    pub fn write(&mut self, message: &str) {
        writeln!(self.log, "{}", message).unwrap();
    }
}
