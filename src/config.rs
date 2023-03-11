use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    admins: Vec<u64>,
    channel_register_msg: u64,
    token: String,
}

impl Config {
    pub fn admins(&self) -> &Vec<u64> {
        &self.admins
    }
    pub fn channel_register_msg(&self) -> u64 {
        self.channel_register_msg
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}


const CONFIG_PATH: &str = ".config/config.toml";

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = {
        // Get specified path if any
        let args: Vec<String> = std::env::args().collect();
        let conf_path = if args.len() > 1 {
            &args[1]
        } else {
            CONFIG_PATH
        };
        let conf: Config = match confy::load_path(conf_path) {
            Ok(conf) => conf,
            Err(err) => {
                confy::store_path(conf_path, Config::default()).unwrap();
                panic!("Error while loading config, a config file must be specified to .config/config.toml: {err:?}");
            }
        };
        conf
       
    };
}