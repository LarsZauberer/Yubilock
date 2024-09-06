use clap::Parser;
use log::warn;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Config {
    #[arg(short = 'c', long = "cmd", default_value_t = String::from("swaylock"))]
    pub lock_cmd: String,
    #[arg(short = 'k', long = "keys", default_value_t = String::from(""))]
    pub keys: String,
}

impl Config {
    /// Computes the locking program
    pub fn get_program(&self) -> String {
        let mut split = self.lock_cmd.split(' ');
        split
            .next()
            .expect("Error: lock_cmd command is empty")
            .to_string()
    }

    /// Computes the arguments for the locking command
    pub fn get_args(&self) -> Vec<String> {
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

    pub fn get_keys(&self) -> Vec<u32> {
        if self.keys == "" {
            warn!("No valid key specified");
            return vec![];
        }
        let split = self.keys.split(',');
        let mut keys: Vec<u32> = Vec::with_capacity(10);
        for i in split {
            if let Ok(key) = u32::from_str_radix(i, 10) {
                keys.push(key);
            } else {
                panic!("The key {} is an invalid key", i);
            }
        }
        keys
    }
}
