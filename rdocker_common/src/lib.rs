#![feature(command_access)]
use std::ffi::OsStr;
// TODO: Why doesn't tokio::process expose Output?
pub use std::process::{Command, Output};

use anyhow::{anyhow, Result};

pub trait CommandExt {
    /// Like output() but translates non-zero error codes as Err()
    fn output_strict(&mut self) -> Result<Output>;

    /// Like output() but gives only stdout as String and trims any trailing newlines
    fn output_value(&mut self) -> Result<String>;

    /// Like output_strict() but gives only stdout as String and trims any trailing newlines
    fn output_strict_value(&mut self) -> Result<String>;

    fn display(&self) -> Result<String>;
}

impl CommandExt for Command {
    fn output_strict(&mut self) -> Result<Output> {
        let output = self.output()?;
        if !output.status.success() {
            return Err(anyhow!(
                "Command `{}` finished with non-zero status code: {}\n========== stderr ==========\n{}=========== stdout ===========\n{}",
                self.display()?,
                output
                    .status
                    .code()
                    .expect("somehow status code is undefined after checking status code..."),
                String::from_utf8(output.stderr)?,
                String::from_utf8(output.stdout)?,
            ));
        }
        Ok(output)
    }

    fn output_value(&mut self) -> Result<String> {
        Ok(String::from_utf8(self.output()?.stdout)?
            .trim_end_matches('\n')
            .to_string())
    }

    fn output_strict_value(&mut self) -> Result<String> {
        Ok(String::from_utf8(self.output_strict()?.stdout)?
            .trim_end_matches('\n')
            .to_string())
    }

    fn display(&self) -> Result<String> {
        let program = OsStr::to_str(self.get_program()).unwrap();
        let args = self
            .get_args()
            .into_iter()
            .map(|arg| OsStr::to_str(arg).unwrap())
            .collect::<Vec<_>>()
            .join(" ");
        Ok(format!("{} {}", program, args))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
