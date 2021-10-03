#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use rdocker::EnvConf;
    use rdocker_common::{Command, CommandExt, Output};
    use std::{fs, path::PathBuf, time::Duration};
    use tokio::sync::OnceCell;

    pub async fn start_rdockerd() -> Result<()> {
        // NOTE: Exit code 1 when nothing found
        let existing_rdockerd_pid = Command::new("pgrep")
            .arg("-x")
            .arg("rdockerd")
            .output_value()
            .map_err(|err| anyhow!("Failed to execute pgrep: '{}'", err))?;

        if !existing_rdockerd_pid.is_empty() {
            Command::new("kill")
                .arg("-9")
                .arg(existing_rdockerd_pid)
                .output()
                .map_err(|err| anyhow!("Failed to execute kill: '{}'", err))?;
        }

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
            // .env("RUSTFLAGS", "-Awarnings") causes rebuilds...
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

        // STEP1: Generate a configuation
        rdocker("gen-conf", "test_env")?;

        let file = fs::File::open("../rd_env_conf.test_env.yaml").map_err(|err| {
            anyhow!(
                "Configuration file not found after should have been generated: '{}'",
                err
            )
        })?;
        // Reading process already checks IPs since it parses them to IpAddr
        let conf: EnvConf = serde_yaml::from_reader(file)?;

        assert_eq!(conf.env_id, "test_env");
        assert_eq!(conf.local_user, "vscode");
        assert_eq!(conf.local_path, PathBuf::from("/workspaces/idea-nursery"));
        assert_eq!(conf.remote_user, "vscode");
        assert_eq!(conf.remote_path, PathBuf::from("/tmp/\"idea-nursery\""));

        // STEP2: Register new env
        // rdocker("set-up-env", "test-env")?;

        // STEP3: Check if env actually registered
        // let output = rdocker("read-env", "test_env")?;
        // assert_eq!(output, "");

        // Cleanup
        fs::remove_file("../rd_env_conf.test_env.yaml")?;

        Ok(())
    }
}
