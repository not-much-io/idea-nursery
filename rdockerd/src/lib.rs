use rdocker_model::rdocker::r_docker_server::RDocker;
pub use rdocker_model::rdocker::r_docker_server::RDockerServer;
use rdocker_model::rdocker::{EchoResponse, EchoRequest};
use tonic::Response;

#[derive(Debug, Default)]
pub struct RDockerService {}

#[tonic::async_trait]
impl RDocker for RDockerService {
    async fn echo(
        &self,
        request: tonic::Request<EchoRequest>,
    ) -> Result<tonic::Response<EchoResponse>, tonic::Status> {
        let reply = EchoResponse {
            message: request.into_inner().message,
        };
        Ok(Response::new(reply))
    }
    
    async fn setup_in_memory_fs(
            &self,
            request: tonic::Request<rdocker_model::rdocker::SetupInMemoryFsRequest>,
        ) -> Result<tonic::Response<rdocker_model::rdocker::SetupInMemoryFsResponse>, tonic::Status> {
        todo!()
    }

    async fn setup_transparent_proxy(
            &self,
            request: tonic::Request<rdocker_model::rdocker::SetupTransparentProxyRequest>,
        ) -> Result<tonic::Response<rdocker_model::rdocker::SetupTransparentProxyResponse>, tonic::Status> {
        todo!()
    }

    async fn register_env(
            &self,
            request: tonic::Request<rdocker_model::rdocker::RegisterEnvRequest>,
        ) -> Result<tonic::Response<rdocker_model::rdocker::RegisterEnvResponse>, tonic::Status> {
        todo!()
    }

    async fn clear_env(
            &self,
            request: tonic::Request<rdocker_model::rdocker::ClearEnvRequest>,
        ) -> Result<tonic::Response<rdocker_model::rdocker::ClearEnvResponse>, tonic::Status> {
        todo!()
    }
}
