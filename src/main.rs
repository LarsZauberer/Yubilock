use std::process::Command;
use yubikey::Serial;
use yubikey::YubiKey;
use yubilock::Config;

fn main() {
    // Load config
    let config: Config = confy::load("yubilock", None).unwrap();

    if is_valid_yubikey_inserted(&config) {
        // Yubikey was found -> Kill the lock screen session
        println!(
            "Valid yubikey found -> Killing locker {}",
            config.get_program()
        );
        let locker = config.get_program();
        Command::new("pkill")
            .arg("--signal")
            .arg("SIGUSR1")
            .arg(locker)
            .output()
            .expect("Error while killing");
    } else {
        // No valid yubikey was found
        println!(
            "No valid yubikey was found -> Spawning the locker {}",
            config.get_program()
        );
        let locker = config.get_program();
        let locker_args = config.get_args();
        let mut c = Command::new(locker);
        for i in locker_args {
            c.arg(i);
        }
        let _ = c.output().expect("Error while running");
    }
}

fn is_valid_yubikey_inserted(config: &Config) -> bool {
    let mut found: bool = false;
    for serial in &config.keys {
        if let Ok(_) = YubiKey::open_by_serial(Serial::from(serial.clone())) {
            // Found an authorized yubikey
            found = true;
            break;
        }
    }

    found
}
