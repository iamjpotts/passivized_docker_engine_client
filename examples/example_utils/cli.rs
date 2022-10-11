use core::future::Future;
use std::process::ExitCode;
use log::*;
use simple_logger::SimpleLogger;
use super::errors::ExampleError;

pub async fn run<F, Fut>(f: F) -> ExitCode
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), ExampleError>>
{
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    match f().await {
        Err(e) => {
            warn!("Failed: {:?}", e);
            ExitCode::FAILURE
        }
        Ok(_) => {
            info!("Done.");
            ExitCode::SUCCESS
        }
    }
}