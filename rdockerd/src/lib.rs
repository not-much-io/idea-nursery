use anyhow::{anyhow, Error, Result};
use log::info;
use rdocker_model::{RDRequest, RDResponse, ResponseContent};
use serde_json as json;
use std::{
    convert::{TryFrom, TryInto},
    future::{ready, Ready},
    net::SocketAddr,
    sync::Arc,
};
use thrussh::ChannelId;
use thrussh::{server as ssh_server, CryptoVec};
use thrussh_keys as ssh_keys;

#[derive(Clone)]
pub struct RDServer {}

impl RDServer {
    pub async fn new(socket_addr: SocketAddr) -> Result<Self> {
        let username = whoami::username();
        let key_pair =
            ssh_keys::load_secret_key(format!("/home/{}/.ssh/id_ed255519", username), None)?;
        let mut server_config = ssh_server::Config::default();
        server_config
            .keys
            .push(key_pair);

        let server = RDServer {};
        ssh_server::run(Arc::new(server_config), &socket_addr.to_string(), server).await?;
        Ok(RDServer {})
    }
}

impl ssh_server::Server for RDServer {
    type Handler = RDServerHandler;

    // New client connects
    fn new(&mut self, _peer_addr: Option<SocketAddr>) -> Self::Handler {
        RDServerHandler::new()
    }
}

pub struct RDServerHandler {}

impl RDServerHandler {
    fn new() -> Self {
        RDServerHandler {}
    }
}

impl ssh_server::Handler for RDServerHandler {
    type Error = Error;
    type FutureAuth = Ready<Result<(Self, ssh_server::Auth), Error>>;
    type FutureUnit = Ready<Result<(Self, ssh_server::Session), Error>>;
    type FutureBool = Ready<Result<(Self, ssh_server::Session, bool), Error>>;

    fn finished(self, session: ssh_server::Session) -> Self::FutureUnit {
        ready(Ok((self, session)))
    }
    fn finished_auth(self, auth: ssh_server::Auth) -> Self::FutureAuth {
        ready(Ok((self, auth)))
    }
    fn finished_bool(self, b: bool, session: ssh_server::Session) -> Self::FutureBool {
        ready(Ok((self, session, b)))
    }

    fn auth_publickey(self, user: &str, public_key: &ssh_keys::key::PublicKey) -> Self::FutureAuth {
        ready(Ok((self, ssh_server::Auth::Accept)))
    }

    fn data(
        self,
        channel: ChannelId,
        request_data: &[u8],
        mut session: ssh_server::Session,
    ) -> Self::FutureUnit {
        let request: RDRequest = RDRequest::from_slice(request_data).unwrap();
        let response: RDResponse = match request {
            RDRequest::Healthcheck => RDResponse(Ok(ResponseContent::Empty)),
            RDRequest::Echo{msg} => RDResponse(Ok(ResponseContent::Message{msg}))
        };

        session.data(
            channel,
            response
                .to_cryptovec()
                .unwrap(),
        );
        self.finished(session)
    }
}
