#![feature(path_try_exists)]
use std::{collections::HashMap, fs, path::Path, sync::Arc};

use anyhow;
use rdocker_model::rdocker::r_docker_server::RDocker;
pub use rdocker_model::rdocker::r_docker_server::RDockerServer;
use rdocker_model::rdocker::*;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct RDockerService {
    env_registry: Arc<Mutex<HashMap<String, Environment>>>,
}

impl RDockerService {
    async fn get_env_strict(&self, env_id: &str) -> anyhow::Result<Environment> {
        let env_registry = self.env_registry.lock().await;
        match env_registry.get(env_id) {
            None => Err(anyhow::anyhow!(
                "Environment ID '{}' not in registry, real values: '{}'",
                env_id,
                env_registry
                    .keys()
                    .cloned()
                    .into_iter()
                    .collect::<Vec<String>>()
                    .join(", "),
            )),
            Some(env) => Ok(env.clone()),
        }
    }
}

#[tonic::async_trait]
impl RDocker for RDockerService {
    async fn register_env(
        &self,
        request: Request<RegisterEnvRequest>,
    ) -> Result<Response<RegisterEnvResponse>, Status> {
        let env_desc = request
            .into_inner()
            .env_desc
            .ok_or_else(|| Status::invalid_argument("Missing EnvDescriptor"))?;

        let mut env_registry = self.env_registry.lock().await;
        if env_registry.contains_key(&env_desc.env_id) {
            return Err(Status::invalid_argument("Environment ID already in use"));
        }

        let mut env = Environment::default();
        env.desc = Some(env_desc.clone());
        env_registry.insert(env_desc.env_id.clone(), env);

        Ok(Response::new(RegisterEnvResponse::default()))
    }

    async fn set_up_in_memory_fs(
        &self,
        request: Request<SetUpInMemoryFsRequest>,
    ) -> Result<Response<SetUpInMemoryFsResponse>, Status> {
        let env_id = request.into_inner().env_id;
        match self
            .get_env_strict(&env_id)
            .await
        {
            Err(err) => Err(Status::not_found(format!("Env not found: {}", err))),
            Ok(env) => {
                if env.status != environment::Status::Registered.into() {
                    return Err(Status::failed_precondition(format!(
                        "Filesystem setup should already be done, expected env status '{:?}', actual env status '{:?}'",
                        environment::Status::Registered,
                        environment::Status::from_i32(env.status),
                    )));
                }

                let remote_path_string = env
                    .desc
                    .expect("TODO")
                    .remote_path;
                let remote_path = Path::new(&remote_path_string);

                match remote_path.try_exists() {
                    Err(err) => {
                        return Err(Status::failed_precondition(format!(
                            "Unable to check for the existance of remote path {:?}, with error: {}",
                            remote_path, err,
                        )))
                    }
                    Ok(exists) => {
                        if exists {
                            return Err(Status::failed_precondition(
                                "Filesystem setup failed, the defined remote path already exists",
                            ));
                        }
                    }
                }

                let _ = fs::write(remote_path, "");

                Ok(Response::new(SetUpInMemoryFsResponse::default()))
            }
        }
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
        request: Request<ReadEnvRequest>,
    ) -> Result<Response<ReadEnvResponse>, Status> {
        let env_id = request.into_inner().env_id;
        match self
            .get_env_strict(&env_id)
            .await
        {
            Err(err) => Err(Status::not_found(format!("Couldn't read env: {}", err))),
            Ok(env) => Ok(Response::new(ReadEnvResponse { env: Some(env) })),
        }
    }

    async fn list_envs(
        &self,
        _request: Request<ListEnvsRequest>,
    ) -> Result<Response<ListEnvsResponse>, Status> {
        let env_registry = self.env_registry.lock().await;
        let mut env_descs = vec![];
        for env_desc in env_registry.values() {
            env_descs.push(env_desc.clone());
        }

        // Ok(Response::new(ListEnvsResponse { env_descs }))
        todo!()
    }
}
