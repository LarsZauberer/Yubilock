use std::process::Command;
use std::thread;
use std::thread::spawn;
use std::time;
use yubikey::Serial;
use yubikey::YubiKey;
use yubilock::Config;

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
