use std::env;
use std::path::PathBuf;
use serde::Deserialize;
use once_cell::sync::Lazy;
use config::{Config, File, FileFormat};

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub port: u16,
    pub address: String,

    pub gpg: String,
    pub store_dir: String,

    pub extensions_dir: String,
    pub gpg_opts: String,
    pub signing_keys: String,
}

fn default_config() -> Configuration {
    Configuration {
        port: 3030,
        address: "127.0.0.1".to_string(),

        gpg: "gpg".to_string(),
        store_dir: "$HOME/passd".to_string(),

        gpg_opts: "".to_string(),
        signing_keys: "".to_string(),
        extensions_dir: "$HOME/passd/.extensions".to_string(),
    }
}

fn load_config() -> Configuration {
    let config_dir = env::var("PASSD_CONFIG_DIR").unwrap_or_else(|_| {
        dirs_next::config_dir()
            .unwrap_or_else(|| PathBuf::from("./config"))
            .join("passd")
            .to_string_lossy()
            .to_string()
    });

    let config_path = format!("{}/config.conf", config_dir);

    let builder = Config::builder().add_source(File::new(&config_path, FileFormat::Ini));

    match builder.build() {
        Ok(configuration) => {
            println!("Successfully loaded configuration");
            configuration.try_deserialize::<Configuration>().unwrap_or_else(|_| default_config())
        }
        Err(err) => {
            eprintln!("Failed to load configuration: {}", err);
            default_config()
        }
    }
}

pub static CONFIG: Lazy<Configuration> = Lazy::new(|| load_config());
