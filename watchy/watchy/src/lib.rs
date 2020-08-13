use parking_lot::{Mutex, MutexGuard};
use std::process::Command;
use std::sync::Arc;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

pub struct Watcher {
    cmd_to_monitor: Command,
    cmd_to_trigger: Command,
    interval:       Duration,
    is_running:     bool,
}

impl Watcher {
    pub fn new(cmd_to_monitor: Command, cmd_to_trigger: Command) -> Watcher {
        Watcher {
            cmd_to_monitor,
            cmd_to_trigger,

            is_running: false,
            interval: Duration::new(1, 0),
        }
    }

    pub fn interval(&mut self, interval: Duration) -> &mut Watcher {
        self.interval = interval;
        self
    }

    pub fn watch(mut self) -> (Arc<Mutex<Watcher>>, JoinHandle<()>) {
        self.is_running = true;

        let arc_mutex_self = Arc::new(Mutex::new(self));
        let arc_mutex_self_clone = Arc::clone(&arc_mutex_self);

        let handle = spawn(move || {
            let mut previous_output = Vec::new();
            loop {
                let mut watcher = arc_mutex_self.lock();
                if !watcher.is_running {
                    break;
                }

                let out = watcher.cmd_to_monitor.output().unwrap();
                if out.stdout != previous_output {
                    watcher.cmd_to_trigger.spawn().unwrap();
                }
                previous_output = out.stdout;

                sleep(watcher.interval);
                MutexGuard::unlock_fair(watcher);
            }
        });

        (arc_mutex_self_clone, handle)
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[tokio::test(core_threads = 2)]
    async fn test_actual_watcher() {
        let trigger_created_file_name = "trigger_created_file";

        let mut sec_since_epoch = Command::new("date");
        sec_since_epoch.arg("+%s");

        let mut echo_done = Command::new("touch");
        echo_done.arg(trigger_created_file_name);

        let interval = Duration::from_millis(25);

        let mut watcher = Watcher::new(sec_since_epoch, echo_done);
        watcher.interval(interval);

        let (watcher_mutex, handle) = watcher.watch();
        sleep(Duration::from_millis(50));
        watcher_mutex.lock().stop();
        handle.join().expect("Join on watcher thread failed");

        let status_code = Command::new("rm")
            .arg(trigger_created_file_name)
            .output()
            .expect("Check of trigger created file failed to run")
            .status
            .code()
            .unwrap();

        assert_eq!(
            status_code, 0,
            "Watcher trigger command didn't run or didn't create a file successfully"
        );
    }
}
