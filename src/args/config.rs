use clap:: {
  Args,
  Subcommand
};
use directories::ProjectDirs;
use serde::Deserialize;
use std::path::Path;
use toml::{map::Map, Value};
// use crate::{auth, args::config};

#[derive(Debug, Args)]
pub struct ConfigCommand {
	#[clap(subcommand)]
	pub command: ConfigSubCommands,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommands{
	/// Change url of the api
	Loc(LocationEntity),
}

#[derive(Debug, Args)]
pub struct LocationEntity{
    /// link to your api (example: "https://your.cool/api")
    pub url: String
}

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub location: String,
	pub key: String,
	pub config_dir: String,
    pub app_id: String,
	pub app_key: String,
    pub client_key: String
}

pub async fn handle(conf_struct: &ConfigCommand, proj_dir: ProjectDirs, config_file: ConfigFile) {
    // auth::check_auth(&config_file).await.expect("Authentication Error");
    match &conf_struct.command {
        ConfigSubCommands::Loc(loc_struct) => handle_location(loc_struct, proj_dir.config_dir(), config_file)
    }
}

pub fn handle_location(loc_struct: &LocationEntity, config_path: &Path, config_file: ConfigFile) {
    let v: Vec<(String, String)> = vec![
        ("location".into(), loc_struct.url.to_string()),
        ("key".into(), config_file.key.into()),
        ("config_dir".into(), config_file.config_dir.into()),
        ("app_id".into(), config_file.app_id.into()),
        ("app_key".into(), config_file.app_key.into()),
        ("client_key".into(), config_file.client_key.into())
    ];

    std::fs::write(config_path.join("locksmith.toml"), to_toml(v).to_string()).expect("Failed to write config file");

    println!("Location has been set! If you want to add appId, appKey, or clientkey, you should edit the config file located at {}", config_path.display());
}

pub fn check_dirs() -> Result<ProjectDirs, ()> {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "meloencoding", "locksmith"){
        let config_path: &Path = proj_dirs.config_dir();
        let config_file: Result<String, std::io::Error> = std::fs::read_to_string(proj_dirs.config_dir().join("locksmith.toml"));
        match config_file {
            Ok(_) => return Ok(proj_dirs),
            Err(_) => {
                reset(&config_path);
                return Ok(proj_dirs); 
            }
        }   
    }
    else {
        return Err(display_error("Error loading project directories".to_string())); 
    }
}

pub fn reset(config_path: &Path) {

        std::fs::create_dir_all(config_path).unwrap();

        let v: Vec<(String, String)> = vec![
            ("location".into(), "https://your.crazy/api".into()),
            ("key".into(), "".into()),
            ("config_dir".into(), config_path.join("locksmith.toml").into_os_string().into_string().unwrap()),
            ("app_id".into(), "".into()),
            ("app_key".into(), "".into()),
            ("client_key".into(), "".into())
        ];
        let toml_string = toml::to_string(&to_toml(v)).expect("Could not encode TOML value");
        std::fs::write(config_path.join("locksmith.toml"), toml_string).expect("Could not write to file!")
}

pub fn read(config_path: &Path) -> std::io::Result<ConfigFile> {
    let content = std::fs::read_to_string(config_path.join("locksmith.toml"))?;
    let config: ConfigFile = toml::from_str(&content)?;
    Ok(config)
}

fn to_toml(v: Vec<(String, String)>) -> Value {
    let mut settings = Map::new();
    for (argument, value) in v {
        settings.insert(argument.into(), Value::String(value));
    }

    Value::Table(settings)
}

pub fn display_error(error: String) {
    println!("{}", error);
    std::process::exit(0x0);
}
