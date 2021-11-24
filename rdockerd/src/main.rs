#![feature(path_try_exists)]
mod lib;

use lib::{RDockerServer, RDockerService};

use anyhow::Error;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "127.0.0.1:50051".parse()?;
    let rdocker_service = RDockerService::default();

    Server::builder()
        .add_service(RDockerServer::new(rdocker_service))
        .serve(addr)
        .await?;

    Ok(())
}
