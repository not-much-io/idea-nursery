mod lib;

use anyhow::{anyhow, Result};
use log::{error, LevelFilter};
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
        lib::CLI::SetUpEnv { env_id } => set_up_env(&env_id).await?,
        lib::CLI::TearDownEnv { env_id } => todo!(),
    }
    Ok(())
}

async fn generate_config(cli_input: lib::GenConfCLI) -> Result<()> {
    let conf = lib::EnvConf::generate(cli_input)
        .await
        .map_err(|err| anyhow!("Failed to construct environment configuration: '{}'", err))?;
    Ok(conf.save_to_file()?)
}

async fn set_up_env(env_id: &str) -> Result<()> {
    let conf = lib::EnvConf::load_from_file(env_id).await?;
    let ctx = lib::Context::new(conf);
    let client = lib::ClientWrapper::new(ctx);

    Ok(())
}
