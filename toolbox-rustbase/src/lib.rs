use anyhow::Result;
use async_trait::async_trait;
use std::process;

#[async_trait]
pub trait CLIProgram<T> {
    fn name(&self) -> &str;

    async fn call(&self) -> Result<process::Output>;

    fn is_installed(&self) -> bool {
        let output = process::Command::new("which")
            .arg(&self.name())
            .output()
            .expect("failed to execute `which` command");

        output.stdout.is_empty()
    }

    async fn parse_output(&self, output: process::Output) -> T;
}
