mod lib;

use anyhow::{anyhow, Result};
use log::{error, LevelFilter};
use rdocker_model::rdocker::EnvDescriptor;
use serde::Serialize;
use serde_yaml;
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
        lib::CLI::ReadEnv { env_id } => read_env(&env_id).await?,
        lib::CLI::ListEnvs { env_id } => todo!(),
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
    lib::ClientWrapper::new(ctx)
        .await?
        .register_env()
        .await?;

    Ok(())
}

async fn read_env(env_id: &str) -> Result<()> {
    let conf = lib::EnvConf::load_from_file(env_id).await?;
    let ctx = lib::Context::new(conf);
    let env_desc = lib::ClientWrapper::new(ctx)
        .await?
        .read_env()
        .await?;
    let env_desc_string = serde_yaml::to_string(&SerializableEnvDescriptor::new(env_desc))?;
    println!("{}", env_desc_string);

    Ok(())
}

#[derive(Serialize)]
struct SerializableEnvDescriptor {
    env_id:      String,
    local_ip:    String,
    local_user:  String,
    local_path:  String,
    remote_ip:   String,
    remote_user: String,
    remote_path: String,
}

impl SerializableEnvDescriptor {
    fn new(env_desc: EnvDescriptor) -> Self {
        Self {
            env_id:      env_desc.env_id,
            local_ip:    env_desc.local_ip,
            local_user:  env_desc.local_user,
            local_path:  env_desc.local_path,
            remote_ip:   env_desc.remote_ip,
            remote_user: env_desc.remote_user,
            remote_path: env_desc.remote_path,
        }
    }
}
