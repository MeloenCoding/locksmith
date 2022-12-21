mod args;

use clap::Parser;

use args::{set, list, config::{self, ConfigFile}, gen, get, EntityType, auth};
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    print_message();

    auth::check_auth();

    let proj_dir: ProjectDirs = config::check_dirs().unwrap();

    let args: args::LocksmithArgs = args::LocksmithArgs::parse();

    let config_file: ConfigFile = config::read_config(proj_dir.config_dir()).unwrap();
    match &args.entity_type {
        EntityType::Config(config_struct) => {config::handle_config(config_struct);},
        EntityType::Gen(gen_struct) => {gen::handle_gen(gen_struct);},
        EntityType::Get(get_struct) => {get::handle_get(get_struct, config_file).await.expect("Request Error");},
        EntityType::List(list_struct) => {list::handle_list(list_struct, config_file).await.expect("Request Error");},
        EntityType::Set(set_struct) => {set::handle_set(set_struct, config_file).await.expect("Request Error");}
    }
}

fn print_message() {
    println!("
    █    ████▄ ▄█▄    █  █▀  ▄▄▄▄▄   █▀▄▀█ ▄█    ▄▄▄▄▀ ▄  █
    █    █   █ █▀ ▀▄  █▄█   █     ▀▄ █ █ █ ██ ▀▀▀ █   █   █
    █    █   █ █   ▀  █▀▄ ▄  ▀▀▀▀▄   █ ▄ █ ██     █   ██▀▀█   v{} - by {}    
    ███▄ ▀████ █▄  ▄▀ █  █ ▀▄▄▄▄▀    █   █ ▐█    █    █   █        
        ▀      ▀███▀    █               █   ▐   ▀        █
                         ▀               ▀                ▀ ", 
    env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
}
