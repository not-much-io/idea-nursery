#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::Duration;

    use anyhow::{anyhow, Result};
    use rdocker_model::{RDRequest, RDResponse};
    use tokio::sync::OnceCell;

    // TODO: Contains hostname
    static SOCKET_ADDR: &str = "127.0.0.1:2222";
    const TEST_PUBLIC_KEY: &str = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIBkG85mxAKLLzh5gTCK2STCtXhgc7Usy3tiTAjIwX5df vscode@70b228b58266";
    const TEST_PRIVATE_KEY: &str = "-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
QyNTUxOQAAACAZBvOZsQCiy84eYEwitkkwrV4YHO1LMt7YkwIyMF+XXwAAAJgjLNvHIyzb
xwAAAAtzc2gtZWQyNTUxOQAAACAZBvOZsQCiy84eYEwitkkwrV4YHO1LMt7YkwIyMF+XXw
AAAEA9w9AZnQ3YbMp+cO/24EO/6BGdGbXte+JXhdd1RoN06xkG85mxAKLLzh5gTCK2STCt
Xhgc7Usy3tiTAjIwX5dfAAAAE3ZzY29kZUA3MGIyMjhiNTgyNjYBAg==
-----END OPENSSH PRIVATE KEY-----";

    fn setup_keypair() -> Result<()> {
        fs::write(
            format!("/home/{}/.ssh/id_ed255519", whoami::username()),
            TEST_PRIVATE_KEY,
        )?;
        fs::write(
            format!("/home/{}/.ssh/id_ed255519.pub", whoami::username()),
            TEST_PUBLIC_KEY,
        )?;
        Ok(())
    }

    async fn setup_server() -> Result<()> {
        tokio::spawn(rdockerd::RDServer::new(SOCKET_ADDR.parse()?));
        tokio::time::sleep(Duration::from_millis(250)).await;
        Ok(())
    }

    async fn setup_tests() -> Result<()> {
        setup_keypair()?;
        setup_server().await?;
        Ok(())
    }

    async fn once_init_tests() -> Result<()> {
        static ONCE_INIT_TESTS: OnceCell<Result<()>> = OnceCell::const_new();
        ONCE_INIT_TESTS
            .get_or_init(setup_tests)
            .await
            .as_ref()
            .map(|_| ()) // Don't care about the val but need to own
            .map_err(|err| anyhow!("Failed to initialize tests with error: '{}'", err))
    }

    #[tokio::test]
    async fn test_healthcheck() -> Result<()> {
        once_init_tests().await?;

        rdocker::RDClient::new(SOCKET_ADDR.parse()?)
            .await?
            .healthcheck()
            .await?;

        Ok(())
    }
}
