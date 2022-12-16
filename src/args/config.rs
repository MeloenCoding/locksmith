use clap:: {
  Args,
  Subcommand
};
use directories::ProjectDirs;
use serde::Deserialize;
use std::path::Path;
use toml::{map::Map, Value};


#[derive(Debug, Args)]
pub struct ConfigCommand {
	#[clap(subcommand)]
	pub command: ConfigSubCommands,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommands{
	/// Change location of the storage. Localy or Remote
	Loc(LocationEntity),
}

#[derive(Debug, Args)]
pub struct LocationEntity{
    /// link to your api (example: "https://your.cool/api")
    #[arg(short)]
    pub location: Option<String>
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

pub fn handle_config(conf_struct: &ConfigCommand) {
    dbg!(conf_struct);
}

pub fn check_dirs() -> Result<ProjectDirs, ()> {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "meloencoding", "locksmith"){
        let config_path: &Path = proj_dirs.config_dir();
        let config_file = std::fs::read_to_string(proj_dirs.config_dir().join("locksmith.toml"));
        match config_file {
            Ok(_) => return Ok(proj_dirs),
            Err(_) => {
                reset_config(&config_path);
                return Ok(proj_dirs); 
            }
        }   
    }
    else {
        return Err(display_error("Error loading project directories".to_string())); 
    }
}

pub fn reset_config(config_path: &Path) {

        std::fs::create_dir_all(config_path).unwrap();

        let v: Vec<(String, String)> = vec![
            ("location".into(), config_path.join("locksmith.toml").into_os_string().into_string().unwrap()),
            ("key".into(), "".into()),
            ("config_dir".into(), "https://your.crazy/api".into()),
            ("app_id".into(), "".into()),
            ("app_key".into(), "".into()),
            ("client_key".into(), "".into())
        ];
        let toml_string = toml::to_string(&to_toml(v)).expect("Could not encode TOML value");
        std::fs::write(config_path.join("locksmith.toml"), toml_string).expect("Could not write to file!")
}

pub fn read_config(config_path: &Path) -> std::io::Result<ConfigFile> {
    let content = std::fs::read_to_string(config_path.join("locksmith.toml"))?;
    dbg!(toml::from_str(&content)?);
    Ok(toml::from_str(&content)?)
}

fn to_toml(v: Vec<(String, String)>) -> Value {
    let mut settings = Map::new();
    for (argument, value) in v {
        settings.insert(argument.into(), Value::String(value));
    }

    let mut map = Map::new();
    map.insert("Settings".into(), Value::Table(settings));
    Value::Table(map)
}


pub fn display_error(error: String) {
    dbg!(error);
    std::process::exit(0x0100);
}
