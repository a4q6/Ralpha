pub mod api_client;
pub mod constants;
pub mod datamodels;
pub mod feedhandler;

// use chrono::Utc;
use flexi_logger::Duplicate;
use flexi_logger::FileSpec;
use flexi_logger::Logger;
use log::info;
// use uuid::Uuid;

// use crate::api_client::execution_client::ExecutionClient;
use crate::feedhandler::bitflyer::bitflyer_socketio::BitFlyerSocketIo;
use crate::feedhandler::ticklogger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Start");

    // set logger
    Logger::try_with_str("info, my::critical::module=trace")?
        .format(flexi_logger::detailed_format)
        .log_to_file(FileSpec::default().directory("testlogs"))
        .duplicate_to_stdout(Duplicate::All)
        .append()
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(3),
        )
        .start()?;

    // set logic
    let mut t_logger = ticklogger::TickLogger::new("bitflyer");

    // set feedhandlers
    let mut bfsocket = BitFlyerSocketIo::new();
    bfsocket.set_callback(Box::new(move |data| t_logger.callback(data)));
    bfsocket.connect(vec![
        "lightning_executions_BTC_JPY".to_string(),
        "lightning_board_snapshot_BTC_JPY".to_string(),
        "lightning_board_BTC_JPY".to_string(),
        // "lightning_ticker_FX_BTC_JPY".to_string(),
    ]);

    // start loop
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        info!("Hearbeat");
    }
}
