mod lib;

use anyhow::Result;
use log::{error, LevelFilter};
use rdocker_model::rdocker::EchoRequest;
use structopt::StructOpt;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    let cli = lib::CLI::from_args();
    let conf = lib::EnvConfig {
        env_id:      todo!(),
        local_ip:    todo!(),
        local_user:  todo!(),
        local_path:  todo!(),
        remote_ip:   todo!(),
        remote_user: todo!(),
        remote_path: todo!(),
    };
    let ctx = lib::Context::new(conf).await?;
    let client = lib::ClientWrapper::new(ctx).await?;
    if let Err(err) = try_main(client).await {
        error!("{}", err);
        std::process::exit(1);
    };

    Ok(())
}

async fn try_main(_client: lib::ClientWrapper) -> Result<()> {
    let _request = Request::new(EchoRequest {
        message: "echo".into(),
    });

    Ok(())
}
