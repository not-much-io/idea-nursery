use std::{collections::HashMap, sync::Arc};

use rdocker_model::rdocker::r_docker_server::RDocker;
pub use rdocker_model::rdocker::r_docker_server::RDockerServer;
use rdocker_model::rdocker::*;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct RDockerService {
    env_registry: Arc<Mutex<HashMap<String, EnvDescriptor>>>,
}

#[tonic::async_trait]
impl RDocker for RDockerService {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let reply = EchoResponse {
            message: request.into_inner().message,
        };
        Ok(Response::new(reply))
    }

    async fn register_env(
        &self,
        request: Request<RegisterEnvRequest>,
    ) -> Result<Response<RegisterEnvResponse>, Status> {
        let message = request.into_inner();
        let env_desc = message
            .env_desc
            .ok_or_else(|| Status::invalid_argument("Missing EnvDescriptor"))?;

        self.env_registry
            .lock()
            .await
            .insert(env_desc.env_id.clone(), env_desc);

        todo!()
    }

    async fn read_env(
        &self,
        _request: Request<ReadEnvRequest>,
    ) -> Result<Response<ReadEnvResponse>, Status> {
        todo!()
    }

    async fn list_envs(
        &self,
        _request: Request<ListEnvsRequest>,
    ) -> Result<Response<ListEnvsResponse>, Status> {
        todo!()
    }

    async fn unregister_env(
        &self,
        _request: Request<UnregisterEnvRequest>,
    ) -> Result<Response<UnregisterEnvResponse>, Status> {
        todo!()
    }

    async fn setup_in_memory_fs(
        &self,
        _request: Request<SetupInMemoryFsRequest>,
    ) -> Result<Response<SetupInMemoryFsResponse>, Status> {
        todo!()
    }

    async fn setup_fs_sync(
        &self,
        _request: tonic::Request<rdocker_model::rdocker::SetupFsSyncRequest>,
    ) -> Result<tonic::Response<rdocker_model::rdocker::SetupFsSyncResponse>, tonic::Status> {
        todo!()
    }

    async fn setup_transparent_proxy(
        &self,
        _request: Request<SetupTransparentProxyRequest>,
    ) -> Result<Response<SetupTransparentProxyResponse>, Status> {
        todo!()
    }
}
