mod lib;

use anyhow::Result;
use log::{error, info, LevelFilter};

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    if let Err(err) = try_main() {
        error!("{}", err);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    info!("starting up rdocker...");
    info!("collecting environment_data...");
    Ok(())
}
