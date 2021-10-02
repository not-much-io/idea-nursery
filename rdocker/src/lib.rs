use std::{env, net::IpAddr, path::PathBuf};

use anyhow::Result;
use rdocker_model::rdocker::{
    env_descriptor, r_docker_client::RDockerClient, EnvDescriptor, RegisterEnvRequest,
};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use tokio::process::Command as ProcessCommand;
use tonic::transport::Channel;

/// TODO
#[derive(StructOpt)]
#[structopt(name = "rdocker")]
pub enum CLI {
    /// Generate a configuration file
    GenerateConfig(EnvConfig),

    /// Set up env, matching configuration file must exist
    SetUpEnv {
        /// TODO
        #[structopt(long)]
        env_id: String,
    },

    /// Tear down env, matching configuration file must exist
    TearDownEnv {
        /// TODO
        #[structopt(long)]
        env_id: String,
    },
}

/// Configuration of one environment
#[derive(StructOpt, Serialize, Deserialize, Debug)]
pub struct EnvConfig {
    /// A unique identifier for an environment in remote
    #[structopt(long)]
    pub env_id: String,

    /// IP of the local machine, default to current local ip
    #[structopt(long)]
    pub local_ip:   Option<IpAddr>,
    /// Username on the local machine, default to current session username
    #[structopt(long)]
    pub local_user: Option<String>,
    /// Path to the project of interest on local machine, default to working directory
    #[structopt(long)]
    pub local_path: Option<PathBuf>,

    /// IP of the remote machine, defaults to ip in DOCKER_HOST env variable
    #[structopt(long)]
    pub remote_ip:   Option<IpAddr>,
    /// Username on the remote machine, defaults to username in DOCKER_HOST env variable
    #[structopt(long)]
    pub remote_user: Option<String>,
    /// Path to the target dir on the remote machine, defaults to /tmp/{curr_dir_name}
    #[structopt(long)]
    pub remote_path: Option<PathBuf>,
}

/// The context of one execution of rdocker
/// Contains data and state that will be passed along one run of rdocker
#[derive(Clone)]
pub struct Context {
    pub env_id: String,

    pub local_ip:   IpAddr,
    pub local_user: String,
    pub local_path: PathBuf,

    pub remote_ip:   IpAddr,
    pub remote_user: String,
    pub remote_path: PathBuf,
}

impl Context {
    pub async fn new(conf: EnvConfig) -> Result<Self> {
        // Example: ssh://username@192.0.2.1
        let docker_host = env::var("DOCKER_HOST")?;
        // TODO: validate

        let env_id = conf.env_id;

        let local_ip = match conf.local_ip {
            Some(ip) => ip,
            None => Self::default_local_ip().await?,
        };
        let local_user = match conf.local_user {
            Some(user) => user,
            None => Self::default_local_user().await?,
        };
        let local_path = match conf.local_path {
            Some(path) => path,
            None => Self::default_local_path()?,
        };

        let remote_ip = match conf.remote_ip {
            Some(ip) => ip,
            None => Self::default_remote_ip(&docker_host)?,
        };
        let remote_user = match conf.remote_user {
            Some(user) => user,
            None => Self::default_remote_user(&docker_host)?,
        };
        let remote_path = match conf.remote_path {
            Some(path) => path,
            None => Self::default_remote_path()?,
        };

        Ok(Self {
            env_id,
            local_ip,
            local_user,
            local_path,
            remote_ip,
            remote_user,
            remote_path,
        })
    }

    // TODO: Implement something more robust and generic
    async fn default_local_ip() -> Result<IpAddr> {
        let out = ProcessCommand::new("ipconfig")
            .arg("getifaddr")
            .arg("en0")
            .output()
            .await?
            .stdout;
        Ok(String::from_utf8(out)?.parse()?)
    }

    async fn default_local_user() -> Result<String> {
        let out = ProcessCommand::new("whoami")
            .output()
            .await?
            .stdout;
        Ok(String::from_utf8(out)?)
    }

    fn default_local_path() -> Result<PathBuf> {
        Ok(env::current_dir()?)
    }

    // TODO: Use a nice regex? Handle errors.
    fn default_remote_ip(docker_host: &str) -> Result<IpAddr> {
        Ok(docker_host
            .split('@')
            .last()
            .expect("can't parse DOCKER_HOST variable for ip")
            .parse()?)
    }

    // TODO: Use a nice regex? Handle errors.
    fn default_remote_user(docker_host: &str) -> Result<String> {
        Ok(docker_host
            .split('@')
            .next()
            .expect("can't parse DOCKER_HOST variable for username")
            .split('/')
            .last()
            .expect("can't parse DOCKER_HOST variable for username")
            .parse()?)
    }

    // TODO: Handle errors
    fn default_remote_path() -> Result<PathBuf> {
        Ok(PathBuf::from(format!(
            "/tmp/{:?}",
            env::current_dir()?
                .file_name()
                .unwrap()
        )))
    }
}

pub struct ClientWrapper {
    ctx:   Context,
    inner: RDockerClient<Channel>,
}

impl ClientWrapper {
    pub async fn new(ctx: Context) -> Result<Self> {
        let server_address = format!("http://{}:50051", ctx.remote_ip);
        let inner = RDockerClient::connect(server_address).await?;
        Ok(Self { ctx, inner })
    }

    pub async fn setup_env(&mut self) -> Result<()> {
        self.register_env().await?;
        todo!()
    }

    pub async fn teardown_env(&mut self) -> Result<()> {
        todo!()
    }

    async fn register_env(&mut self) -> Result<()> {
        let ctx = self.ctx.clone();
        let env_desc = EnvDescriptor {
            env_id: ctx.env_id,

            local_ip:   ctx.local_ip.to_string(),
            local_user: ctx.local_user.to_string(),
            local_path: ctx
                .local_path
                .to_str()
                .unwrap()
                .into(),

            remote_ip:   ctx.remote_ip.to_string(),
            remote_user: ctx.remote_user.to_string(),
            remote_path: ctx
                .remote_path
                .to_str()
                .unwrap()
                .into(),

            status: env_descriptor::Status::Unregistered.into(),
            error:  env_descriptor::Error::None.into(),
        };

        self.inner
            .register_env(RegisterEnvRequest {
                env_desc: Some(env_desc),
            })
            .await?;

        Ok(())
    }
}
