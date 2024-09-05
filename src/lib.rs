use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub sync_delay: u64,
    pub lock_cmd: String,
    pub keys: Vec<u32>,
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
}
