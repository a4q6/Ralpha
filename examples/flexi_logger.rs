extern crate flexi_logger;
extern crate log;

use flexi_logger::{FileSpec, Logger, WriteMode};
use log::{error, info, warn};

fn main() -> Result<(), flexi_logger::FlexiLoggerError> {
    let logger = Logger::try_with_str("info, my::critical::module=trace")?
        .log_to_file(FileSpec::default().directory("testlogs"))
        .format(flexi_logger::detailed_format)
        .append()
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(3),
        )
        .write_mode(WriteMode::BufferAndFlush)
        .start()?;

    info!("This is an info-level message");
    warn!("This is a warning-level message");
    error!("This is an error-level message");

    logger.flush();

    Ok(())
}
