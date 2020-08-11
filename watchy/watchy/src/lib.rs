use anyhow::Result;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub struct Watcher {
    cmd_to_monitor: Command,
    cmd_to_trigger: Command,

    running:  bool,
    interval: Duration,
}

impl Watcher {
    pub fn new(cmd_to_monitor: Command, cmd_to_trigger: Command) -> Watcher {
        Watcher {
            cmd_to_monitor,
            cmd_to_trigger,

            running: false,
            interval: Duration::new(0, 0),
        }
    }x

    pub fn watch(&mut self) -> Result<()> {
        self.running = true;
        
        let mut previous_output = Vec::new();
        while self.running {
            let out = self.cmd_to_monitor.output()?;
            if out.stdout != previous_output {
                self.cmd_to_trigger.spawn()?;
            }
            previous_output = out.stdout;

            sleep(self.interval);
        }

        Ok(())
    }
}
