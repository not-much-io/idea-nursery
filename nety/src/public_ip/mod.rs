pub mod dig;

use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;
use std::net::IpAddr;
use thiserror::Error;
use toolbox_rustbase::CLIProgram;

// NOTE: In reality a device can have multiple public IPs.
// 	Currently not implemented, would fail to parse.
pub type GetPublicIPResult = Result<IpAddr>;

#[derive(Error, Debug)]
pub enum GetPublicIPError {
    #[error("Parsing public IP failed. Output:\n{0}")]
    IpParsingFailed(String),
}

#[async_trait]
pub trait GetPublicIP: CLIProgram<GetPublicIPResult> + Sync {
    async fn get_public_ip(&self) -> GetPublicIPResult {
        self.parse_output(self.call().await?).await
    }
}
