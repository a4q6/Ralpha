use chrono::Utc;
use file_rotate::{
    compression::Compression, suffix::AppendTimestamp, suffix::FileLimit, ContentLimit, FileRotate,
    TimeFrequency,
};
use std::io::Write;

fn main() {
    let mut log = FileRotate::new(
        "tickerplant/test/log.txt",
        AppendTimestamp::default(FileLimit::MaxFiles(14)),
        ContentLimit::Time(TimeFrequency::Daily),
        Compression::OnRotate(10),
        None,
    );

    log.write_all(b"Hello, world!\n").unwrap();
    // Write a bunch of lines
    for _ in 2..=100 {
        writeln!(log, "{}", Utc::now().to_string()).unwrap();
    }
}
