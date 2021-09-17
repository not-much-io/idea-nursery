use anyhow::{anyhow, Error, Result};
use log::info;
use rdocker_model::{RDRequest, RDResponse, ResponseContent};
use std::future::{ready, Ready};
use std::net::SocketAddr;
use std::sync::Arc;
use thrussh::client as ssh_client;
use thrussh::ChannelMsg;
use thrussh_keys as ssh_keys;

pub struct RDClient {
    channel: ssh_client::Channel,
}

impl RDClient {
    pub async fn new(socket_addr: SocketAddr) -> Result<Self> {
        let username = whoami::username();
        let key_pair =
            ssh_keys::load_secret_key(format!("/home/{}/.ssh/id_ed255519", username), None)?;
        let config = ssh_client::Config::default();
        let handler = RDClientHandler::new();

        let mut handle = ssh_client::connect(Arc::new(config), socket_addr, handler).await?;
        let is_authenticated = handle
            .authenticate_publickey(username, Arc::new(key_pair))
            .await?;

        if !is_authenticated {
            return Err(anyhow!("Public key authentication failed"));
        }

        let channel = handle
            .channel_open_session()
            .await?;

        Ok(RDClient { channel })
    }

    pub async fn do_req(&mut self, req: RDRequest) -> Result<RDResponse> {
        let req_data: &[u8] = &req.clone().to_vec()?;
        self.channel
            .data(req_data)
            .await
            .map_err(|err| anyhow!("Failed sending request on channel with error: {}", err))?;

        // TODO: Add timeout
        let channel_msg = self
            .channel
            .wait()
            .await
            .ok_or_else(|| anyhow!("Server did no response to request {:?}", req))?;

        match channel_msg {
            ChannelMsg::Data { data } => Ok(RDResponse::from_cryptovec(data)?),
            _ => Err(anyhow!(
                "Server didn't respond with data but '{:?}' instead",
                channel_msg,
            )),
        }
    }

    pub async fn healthcheck(&mut self) -> Result<()> {
        let resp = self
            .do_req(RDRequest::Healthcheck)
            .await?;

            match resp.0 {
                Ok(ResponseContent::Empty) => Ok(()),
                Ok(content) => Err(anyhow!("Unexpected content: {:?}", content)),
                Err(err) => Err(anyhow!("Healthcheck failed with error: {}", err)),
            }
    }

    pub async fn echo(&mut self, send_msg: String) -> Result<()> {
        let resp = self
            .do_req(RDRequest::Echo { msg: send_msg })
            .await?;

        match resp.0 {
            Ok(ResponseContent::Message{msg}) => Ok(()),
            Ok(content) => Err(anyhow!("Unexpected content: {:?}", content)),
            Err(err) => Err(anyhow!("Echo failed with error: {}", err)),
        }
    }
}

struct RDClientHandler {}

impl RDClientHandler {
    pub fn new() -> RDClientHandler {
        RDClientHandler {}
    }
}

impl ssh_client::Handler for RDClientHandler {
    type Error = Error;
    type FutureUnit = Ready<Result<(Self, ssh_client::Session), Error>>;
    type FutureBool = Ready<Result<(Self, bool), Error>>;

    fn finished(self, session: ssh_client::Session) -> Self::FutureUnit {
        info!("ssh_client::Handler.finished");
        ready(Ok((self, session)))
    }
    fn finished_bool(self, b: bool) -> Self::FutureBool {
        info!("ssh_client::Handler.finished_bool");
        ready(Ok((self, b)))
    }

    fn check_server_key(self, server_public_key: &ssh_keys::key::PublicKey) -> Self::FutureBool {
        self.finished_bool(true)
    }
}

