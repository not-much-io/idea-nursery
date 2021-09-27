mod lib;

use anyhow::Result;
use log::{error, info, LevelFilter};
use rdocker_model::rdocker::{r_docker_client::RDockerClient, EchoRequest};
use tonic::Request;

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    if let Err(err) = try_main().await {
        error!("{}", err);
        std::process::exit(1);
    }
}

async fn try_main() -> Result<()> {
    info!("runnign rdocker ...");

    let mut client = RDockerClient::connect("http://127.0.0.1:50051").await?;
    let request = Request::new(EchoRequest {
        message: "echo".into(),
    });
    let response = client.echo(request).await?;

    info!("RESPONSE={:?}", response);
    if response.into_inner().message != "echo" {
        error!("wtf?");
    }

    Ok(())
}
