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
    lock_cmd: Vec<String>,
    keys: Vec<u32>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            sync_delay: 1000,
            lock_cmd: vec!["swaylock".to_string()],
            keys: vec![],
        }
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
                found = true;
                let locker = config.lock_cmd[0].clone();
                let _ = spawn(|| {
                    Command::new("pkill")
                        .arg("--signal")
                        .arg("SIGUSR1")
                        .arg(locker)
                        .output()
                });
            }
        }
        if found {
            continue;
        } else {
            let locker = config.lock_cmd[0].clone();
            let locker_args = config.lock_cmd[1..].to_vec();
            let _ = spawn(|| {
                let mut c = Command::new(locker);
                for i in locker_args {
                    c.arg(i);
                }
                let _ = c.output();
            });
        }
        thread::sleep(time::Duration::from_millis(config.sync_delay));
    }
}
