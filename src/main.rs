mod args;

use std::{path::Path, fs::{self, File}, io::Write};
use clap::Parser;
use directories::ProjectDirs;

use args::{set, list, config, gen, get, EntityType, auth};

#[tokio::main]
async fn main() {
    print_message();

    check_dirs();    

    let args: args::LocksmithArgs = args::LocksmithArgs::parse();
    
    auth::check_auth();
    
    match &args.entity_type {
        EntityType::Config(config_struct) => {config::handle_config(config_struct);},
        EntityType::Gen(gen_struct) => {gen::handle_gen(gen_struct);},
        EntityType::Get(get_struct) => {get::handle_get(get_struct);},
        EntityType::List(list_struct) => {list::handle_list(list_struct).await.expect("Request Error");},
        EntityType::Set(set_struct) => {set::handle_set(set_struct).await.expect("Request Error");}
        EntityType::Auth(auth_struct) => {auth::handle_auth(auth_struct);}
    }
}

fn check_dirs() {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "meloencoding", "locksmith"){
        let config_path: &Path = proj_dirs.config_dir();
        
        let config_file = fs::read_to_string(proj_dirs.config_dir().join("locksmith.toml"));
        
        match config_file {
            Ok(_) => return,
            Err(_) => {
                reset_config(config_path);
            }
        }   
    }
}

pub fn reset_config(config_path: &Path) {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "meloencoding", "locksmith"){ 

        std::fs::create_dir_all(config_path).unwrap();
        let byte_string: String = format!("location = {:?}\nkey = \"\"\nconfig_dir = {:?}\napp_id = \"\"\napp_key = \"\"\n", 
            "https://your.crazy/api", 
            proj_dirs.config_dir().join("locksmith.toml").as_os_str()
        );

        // create config and data file if it doesn'st exist and write some data to it
        let mut new_config_file: File = File::create(proj_dirs.config_dir().join("locksmith.toml"))
            .expect("can't create config file");
            
        new_config_file
            .write_all(byte_string.as_bytes())
            .expect("can't write config file");
    }
}

pub fn display_error(error: String) {
    dbg!(error);
    std::process::exit(0x0100);
}

fn print_message() {
    println!("
    █    ████▄ ▄█▄    █  █▀  ▄▄▄▄▄   █▀▄▀█ ▄█    ▄▄▄▄▀ ▄  █
    █    █   █ █▀ ▀▄  █▄█   █     ▀▄ █ █ █ ██ ▀▀▀ █   █   █
    █    █   █ █   ▀  █▀▄ ▄  ▀▀▀▀▄   █ ▄ █ ██     █   ██▀▀█   v{} - made by {}    
    ███▄ ▀████ █▄  ▄▀ █  █ ▀▄▄▄▄▀    █   █ ▐█    █    █   █        
        ▀      ▀███▀    █               █   ▐   ▀        █
                         ▀               ▀                ▀ ", 
    env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
}
