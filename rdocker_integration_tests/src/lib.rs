#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use rdocker::EnvConf;
    use rdocker_common::{Command, CommandExt};
    use std::{fs, net::IpAddr, path::PathBuf, str::FromStr, time::Duration};
    use tokio::sync::OnceCell;

    pub async fn start_rdockerd() -> Result<()> {
        Command::new("cargo")
            .current_dir("/workspaces/idea-nursery")
            .arg("run")
            .arg("--bin")
            .arg("rdockerd")
            .spawn()
            .map_err(|err| anyhow!("Failed to run rdockerd: '{}'", err))?;

        // TODO: Be smarter about this
        tokio::time::sleep(Duration::from_millis(500)).await;

        Ok(())
    }

    pub async fn test_setup() -> Result<()> {
        start_rdockerd()
            .await
            .map_err(|err| anyhow!("Failed to setup tests: {}", err))
    }

    pub async fn once_init_tests() -> Result<()> {
        static ONCE_INIT_TESTS: OnceCell<Result<()>> = OnceCell::const_new();
        ONCE_INIT_TESTS
            .get_or_init(test_setup)
            .await
            .as_ref()
            .map(|_| ())
            .map_err(|err| anyhow!("Failed to once init tests: {}", err))
    }

    fn rdocker(subcommand: &str, env_id: &str) -> Result<String> {
        Command::new("cargo")
            .env("DOCKER_HOST", "ssh://vscode@127.0.0.1")
            .current_dir("/workspaces/idea-nursery")
            .arg("run")
            .arg("--bin")
            .arg("rdocker")
            .arg("--")
            .arg(subcommand)
            .arg("--env-id")
            .arg(env_id)
            .output_strict_value()
    }

    #[tokio::test]
    pub async fn test_e2e() -> Result<()> {
        once_init_tests().await?;

        generate_and_check_conf()?;
        register_and_check_new_env()?;

        // Cleanup
        fs::remove_file("../rd_env_conf.test_env.yaml")?;
        Ok(())
    }

    fn generate_and_check_conf() -> Result<()> {
        rdocker("gen-conf", "test_env")?;

        let new_conf_file = fs::File::open("../rd_env_conf.test_env.yaml").map_err(|err| {
            anyhow!(
                "Configuration file not found after should have been generated: '{}'",
                err
            )
        })?;
        let env_conf: EnvConf = serde_yaml::from_reader(new_conf_file)?;
        check_env_conf(env_conf)
    }

    fn register_and_check_new_env() -> Result<()> {
        rdocker("set-up-env", "test_env")?;
        let env_conf_output = rdocker("read-env", "test_env")?;
        let env_conf: EnvConf = serde_yaml::from_str(env_conf_output.as_str())?;
        check_env_conf(env_conf)
    }

    fn check_env_conf(env_conf: EnvConf) -> Result<()> {
        assert_eq!(env_conf.env_id, "test_env");
        assert_eq!(env_conf.local_ip, IpAddr::from_str("172.17.0.2")?);
        assert_eq!(env_conf.local_user, "vscode");
        assert_eq!(
            env_conf.local_path,
            PathBuf::from("/workspaces/idea-nursery")
        );
        assert_eq!(env_conf.remote_ip, IpAddr::from_str("127.0.0.1")?);
        assert_eq!(env_conf.remote_user, "vscode");
        assert_eq!(
            env_conf.remote_path,
            PathBuf::from(r#"/tmp/"idea-nursery""#)
        );

        Ok(())
    }
}
