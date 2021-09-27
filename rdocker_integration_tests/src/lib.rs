#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use std::{process::Command, str::from_utf8, time::Duration};
    use tokio::sync::OnceCell;

    pub async fn start_rdockerd() -> Result<()> {
        let pgrep_output = Command::new("pgrep")
            .arg("-x")
            .arg("rdockerd")
            .output()
            .map_err(|err| anyhow!("Failed to pgrep for rdockerd: {}", err))?
            .stdout;
        let pgrep_stdout = from_utf8(&pgrep_output)?;

        if !pgrep_stdout.is_empty() {
            Command::new("kill")
                .arg("-9")
                .arg(pgrep_stdout) // a pid
                .output()
                .map_err(|err| anyhow!("Failed to pkill rdockerd: {}", err))?;
            // TODO: Be smarter about this
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        Command::new("cargo")
            .current_dir("/workspaces/idea-nursery")
            .arg("run")
            .arg("--bin")
            .arg("rdockerd")
            .spawn()
            .map_err(|err| anyhow!("Failed to start rdockerd: {}", err))?;

        // TODO: Be smarter about this
        tokio::time::sleep(Duration::from_millis(100)).await;

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

    #[tokio::test]
    pub async fn test_echo() -> Result<()> {
        once_init_tests().await?;

        let output = Command::new("cargo")
            .current_dir("/workspaces/idea-nursery")
            .arg("run")
            .arg("--bin")
            .arg("rdocker")
            .output()
            .map_err(|err| anyhow!("rdocker run failed: {}", err))?;

        let stderr = from_utf8(&output.stderr)?;
        let stdout = from_utf8(&output.stdout)?;
        if !output.status.success() {
            return Err(anyhow!(
                "running rdocker didn't finish successfully\n
                ========== stdout ==========\n
                {}
                ========== stderr ==========\n
                {}",
                stderr,
                stdout,
            ));
        }

        Ok(())
    }
}
