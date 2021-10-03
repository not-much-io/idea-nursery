use std::{env, net::IpAddr, path::PathBuf};

use anyhow::{anyhow, Result};
use local_ip_address::local_ip;
use rdocker_model::rdocker::{
    env_descriptor, r_docker_client::RDockerClient, EnvDescriptor, RegisterEnvRequest,
};
use serde::{Deserialize, Serialize};
use std::fs;
use structopt::StructOpt;
use tokio::process::Command as ProcessCommand;
use tonic::transport::Channel;

/// TODO
#[derive(StructOpt)]
#[structopt(name = "rdocker")]
pub enum CLI {
    /// Generate a configuration file
    GenConf(GenConfCLI),

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
#[derive(StructOpt, Debug)]
pub struct GenConfCLI {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct EnvConf {
    pub env_id: String,

    pub local_ip:   IpAddr,
    pub local_user: String,
    pub local_path: PathBuf,

    pub remote_ip:   IpAddr,
    pub remote_user: String,
    pub remote_path: PathBuf,
}

impl EnvConf {
    pub async fn new(cli: GenConfCLI) -> Result<Self> {
        // Example: ssh://username@192.0.2.1
        let docker_host = env::var("DOCKER_HOST")
            .map_err(|err| anyhow!("Issue with DOCKER_HOST variable: {}", err))?;

        let env_id = cli.env_id;

        let local_ip = match cli.local_ip {
            Some(ip) => ip,
            None => Self::default_local_ip()
                .await
                .map_err(|err| anyhow!("Failed to infer local_ip with error: '{}'", err))?,
        };
        let local_user = match cli.local_user {
            Some(user) => user,
            None => Self::default_local_user()
                .await
                .map_err(|err| anyhow!("Failed to infer local_user with error: '{}'", err))?,
        };
        let local_path = match cli.local_path {
            Some(path) => path,
            None => Self::default_local_path()
                .map_err(|err| anyhow!("Failed to infer local_path with error: '{}'", err))?,
        };

        let remote_ip = match cli.remote_ip {
            Some(ip) => ip,
            None => Self::default_remote_ip(&docker_host)
                .map_err(|err| anyhow!("Failed to infer remote_ip with error: '{}'", err))?,
        };
        let remote_user = match cli.remote_user {
            Some(user) => user,
            None => Self::default_remote_user(&docker_host)
                .map_err(|err| anyhow!("Failed to infer remote_user with error: '{}'", err))?,
        };
        let remote_path = match cli.remote_path {
            Some(path) => path,
            None => Self::default_remote_path()
                .map_err(|err| anyhow!("Failed to infer remote_path with error: '{}'", err))?,
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

    pub async fn load_from_file(env_id: &str) -> Result<Self> {
        let file = fs::File::open(Self::env_conf_file_name(env_id)).map_err(|err| {
            anyhow!(
                "Failed to open configuration file for env '{}' (expected: {})",
                env_id,
                err,
            )
        })?;
        Ok(serde_yaml::from_reader(file)?)
    }

    pub fn save_to_file(&self) -> Result<()> {
        let file = fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(Self::env_conf_file_name(&self.env_id))
            .map_err(|err| {
                anyhow!(
                    "Error from opening file for writing configuration file: '{}'",
                    err
                )
            })?;

        Ok(serde_yaml::to_writer(file, &self)?)
    }

    // TODO: Implement something more robust and generic
    async fn default_local_ip() -> Result<IpAddr> {
        Ok(local_ip()?)
    }

    async fn default_local_user() -> Result<String> {
        let mut out = ProcessCommand::new("whoami")
            .output()
            .await?
            .stdout;
        // Remove the last newline from output
        out.remove(out.len() - 1);

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
        // TODO: Clean out unrequired quotes
        Ok(PathBuf::from(format!(
            "/tmp/{:?}",
            env::current_dir()?
                .file_name()
                .expect("can't get current dir name")
        )))
    }

    fn env_conf_file_name(env_id: &str) -> String {
        format!("rd_env_conf.{}.yaml", env_id)
    }
}

/// The context of one execution of rdocker
/// Contains data and state that will be passed along one run of rdocker
#[derive(Clone)]
pub struct Context {
    conf: EnvConf,
}

impl Context {
    pub fn new(conf: EnvConf) -> Self {
        Self { conf }
    }
}

pub struct ClientWrapper {
    ctx:   Context,
    inner: RDockerClient<Channel>,
}

impl ClientWrapper {
    pub async fn new(ctx: Context) -> Result<Self> {
        let server_address = format!("http://{}:50051", ctx.conf.remote_ip);
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
        let env_conf = self.ctx.conf.clone();
        let env_desc = EnvDescriptor {
            env_id: env_conf.env_id,

            local_ip:   env_conf.local_ip.to_string(),
            local_user: env_conf
                .local_user
                .to_string(),
            local_path: env_conf
                .local_path
                .to_str()
                .unwrap()
                .into(),

            remote_ip:   env_conf.remote_ip.to_string(),
            remote_user: env_conf
                .remote_user
                .to_string(),
            remote_path: env_conf
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
