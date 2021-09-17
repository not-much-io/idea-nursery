use anyhow::Result;
use cryptovec::CryptoVec;
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_vec};
use thiserror::Error;

// Specify error with anyhow!()

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum RDRequest {
    Healthcheck,
    Echo{msg: String},
}

impl RDRequest {
    pub fn to_vec(self) -> Result<Vec<u8>> {
        Ok(to_vec(&self)?)
    }

    pub fn from_slice(raw: &[u8]) -> Result<Self> {
        Ok(from_slice(raw)?)
    }

    // TODO: Are the cryptovec variants ever needed?

    pub fn to_cryptovec(self) -> Result<CryptoVec> {
        Ok(CryptoVec::from_slice(&self.to_vec()?))
    }

    pub fn from_cryptovec(raw: CryptoVec) -> Result<Self> {
        RDRequest::from_slice(&raw.to_vec())
    }
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ResponseError {}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseContent {
    Empty,
    Message{msg: String},
}

#[derive(Serialize, Deserialize)]
pub struct RDResponse(pub Result<ResponseContent, ResponseError>);

impl RDResponse {
    pub fn to_vec(self) -> Result<Vec<u8>> {
        Ok(to_vec(&self)?)
    }

    pub fn from_vec(data: Vec<u8>) -> Result<Self> {
        Ok(from_slice(&data)?)
    }

    pub fn to_cryptovec(self) -> Result<CryptoVec> {
        Ok(CryptoVec::from_slice(&self.to_vec()?))
    }

    pub fn from_cryptovec(data: CryptoVec) -> Result<Self> {
        RDResponse::from_vec(data.to_vec())
    }
}
