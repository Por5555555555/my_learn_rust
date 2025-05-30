use std::{fs, str, vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config{
    pub title: String,
    pub version: String,
    #[serde(default = "default_debug_mode")]
    pub debug_mode: bool,
    
    pub commands_config: CommandsConfig,
    #[warn(private_interfaces)]
    pub user: UserConfig
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandsConfig{
    #[serde(default = "default_ls_args")]
    pub default_ls_args: Vec<String>,
    #[serde(default = "default_favorite_command")]
    pub favorite_command: String,
    #[serde(default = "default_alias_sus")]
    pub alias_sus: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserConfig{
    #[serde(default = "default_name")]
    name: String,
    #[serde(default = "default_email")]
    email: String,
    #[serde(default)]
    age: Option<u8>
}

pub fn default_debug_mode() -> bool {
    false
}
pub fn default_ls_args() -> Vec<String>{
    vec!["-l".to_string(),"color=auto".to_string()]
}
pub fn default_favorite_command() -> String{
    "neofetch".to_string()
}
pub fn default_alias_sus() -> String{
    "I use ubuntu bwt".to_string()
}
pub fn default_name() -> String{
    "Suomi".to_string()
}
pub fn default_email() -> String{
    "Suomi@Chan".to_string()
}

pub fn default_ls_args_str() -> Vec<&'static str>{
    vec!["-l", "color=auto"]
}
// fn config

impl Config {
    pub fn load(path: &str) ->Result<Self,Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(path)?;
        let config:Config = toml::from_str(&config_content)?;
        Ok(config)
    }

    pub fn load_and_create_default(path: &str) -> Self{
        match Self::load(path) {
            Ok(config) => {
                println!("Sussec to loading file {}",path);
                config
            }
            Err(e) =>{
                eprint!("Error loading config file {}",e);
                Self::default()
            }
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_content = toml::to_string_pretty(path)?;
        fs::write(path, config_content)?;
        Ok(())
    }
}

impl Default for Config{
    fn default() -> Self {
        Config {
            title: "My Rust CLI Tool".to_string(),
            version: "0.1.0".to_string(),
            debug_mode: default_debug_mode(),
            commands_config: CommandsConfig {
                default_ls_args: default_ls_args(),
                favorite_command: default_favorite_command(),
                alias_sus: default_alias_sus(),
            },
            user: UserConfig {
                name: default_name(),
                email: default_email(),
                age: None,
            },
        }
    }
}