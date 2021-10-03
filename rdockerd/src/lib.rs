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
    async fn register_env(
        &self,
        request: Request<RegisterEnvRequest>,
    ) -> Result<Response<RegisterEnvResponse>, Status> {
        let message = request.into_inner();
        let env_desc = message
            .env_desc
            .ok_or_else(|| Status::invalid_argument("Missing EnvDescriptor"))?;

        let mut env_registry = self.env_registry.lock().await;
        if env_registry.contains_key(&env_desc.env_id) {
            return Err(Status::invalid_argument("Environment ID already is use"));
        }
        env_registry.insert(env_desc.env_id.clone(), env_desc.clone());

        Ok(Response::new(RegisterEnvResponse::default()))
    }

    async fn set_up_in_memory_fs(
        &self,
        _request: Request<SetUpInMemoryFsRequest>,
    ) -> Result<Response<SetUpInMemoryFsResponse>, Status> {
        todo!()
    }

    async fn set_up_fs_sync(
        &self,
        _request: Request<SetUpFsSyncRequest>,
    ) -> Result<Response<SetUpFsSyncResponse>, Status> {
        todo!()
    }

    async fn set_up_transparent_proxy(
        &self,
        _request: Request<SetUpTransparentProxyRequest>,
    ) -> Result<Response<SetUpTransparentProxyResponse>, Status> {
        todo!()
    }

    async fn tear_down_transparent_proxy(
        &self,
        _request: Request<TearDownTransparentProxyRequest>,
    ) -> Result<Response<TearDownTransparentProxyResponse>, Status> {
        todo!()
    }

    async fn tear_down_fs_sync(
        &self,
        _request: Request<TearDownFsSyncRequest>,
    ) -> Result<Response<TearDownFsSyncResponse>, Status> {
        todo!()
    }

    async fn tear_down_in_memory_fs(
        &self,
        _request: Request<TearDownInMemoryFsRequest>,
    ) -> Result<Response<TearDownInMemoryFsResponse>, Status> {
        todo!()
    }

    async fn unregister_env(
        &self,
        _request: Request<UnregisterEnvRequest>,
    ) -> Result<Response<UnregisterEnvResponse>, Status> {
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
}
