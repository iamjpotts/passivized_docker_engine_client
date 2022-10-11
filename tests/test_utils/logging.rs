use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

pub fn enable() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    info!("Hello test!");
}