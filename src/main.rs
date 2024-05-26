use serde::{Deserialize, Serialize};
use std::process::Command;
use std::thread;
use std::thread::spawn;
use std::time;
use yubikey::Serial;
use yubikey::YubiKey;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    sync_delay: u64,
    lock_cmd: String,
    keys: Vec<u32>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            sync_delay: 1000,
            lock_cmd: "swaylock".to_string(),
            keys: vec![],
        }
    }
}

impl Config {
    /// Computes the locking program
    fn get_program(&self) -> String {
        let mut split = self.lock_cmd.split(' ');
        split
            .next()
            .expect("Error: lock_cmd command is empty")
            .to_string()
    }

    /// Computes the arguments for the locking command
    fn get_args(&self) -> Vec<String> {
        let mut split = self.lock_cmd.split(' ');
        let mut args: Vec<String> = Vec::with_capacity(10);
        let _ = split.next(); // Remove the program from the string
        loop {
            if let Some(arg) = split.next() {
                args.push(arg.to_string());
            } else {
                break;
            }
        }
        args
    }
}

fn main() {
    // Load config
    let config: Config = confy::load("yubilock", None).unwrap();

    let checker = spawn(move || check_yubikey_thread(&config));

    let _ = checker.join();

    println!("Application finished execution. This shouldn't happen!");
}

fn check_yubikey_thread(config: &Config) {
    loop {
        let mut found: bool = false;
        for serial in &config.keys {
            if let Ok(_) = YubiKey::open_by_serial(Serial::from(serial.clone())) {
                // Found an authorized yubikey
                found = true;
                let locker = config.get_program();
                let _ = spawn(|| {
                    Command::new("pkill")
                        .arg("--signal")
                        .arg("SIGUSR1")
                        .arg(locker)
                        .output()
                        .expect("Error while killing");
                });
                break;
            }
        }
        if found {
            // Since it was found we do not lock the screen
            continue;
        } else {
            let locker = config.get_program();
            let locker_args = config.get_args();
            let _ = spawn(|| {
                let mut c = Command::new(locker);
                for i in locker_args {
                    c.arg(i);
                }
                let _ = c.output().expect("Error while running");
            });
        }
        thread::sleep(time::Duration::from_millis(config.sync_delay));
    }
}
