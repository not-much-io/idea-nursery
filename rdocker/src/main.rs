mod lib;

use anyhow::{anyhow, Result};
use lib::GenConfCLI;
use log::{error, LevelFilter};
use std::fs::{File, OpenOptions};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    let cli = lib::CLI::from_args();

    if let Err(err) = try_main(cli).await {
        error!("{}", err);
        std::process::exit(1);
    };

    Ok(())
}

async fn try_main(cli: lib::CLI) -> Result<()> {
    match cli {
        lib::CLI::GenConf(cli_input) => generate_config(cli_input).await?,
        lib::CLI::SetUpEnv { env_id } => todo!(),
        lib::CLI::TearDownEnv { env_id } => todo!(),
    }
    Ok(())
}

async fn generate_config(cli_input: GenConfCLI) -> Result<()> {
    let conf = lib::EnvConf::new(cli_input)
        .await
        .map_err(|err| anyhow!("Failed to construct environment configuration: {}", err))?;
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(format!("rd_env_conf.{}.yaml", conf.env_id))
        .map_err(|err| {
            anyhow!(
                "Error from opening file for writing configuration file: '{}'",
                err
            )
        })?;
    serde_yaml::to_writer(file, &conf)?;

    Ok(())
}
