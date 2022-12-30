mod args;

use clap::Parser;

use args::{set, list, config::{self, ConfigFile}, gen, get, EntityType, auth};
use directories::ProjectDirs;

#[tokio::main]
async fn main() {
    let proj_dir: ProjectDirs = config::check_dirs().unwrap();

    let args: args::LocksmithArgs = args::LocksmithArgs::parse();

    let config_file: ConfigFile = config::read(proj_dir.config_dir()).unwrap();
    match &args.entity_type {
        EntityType::Config(config_struct) => {config::handle(config_struct, proj_dir, config_file).await;},
        EntityType::Gen(gen_struct) => {gen::handle(gen_struct, config_file).await;},
        EntityType::Get(get_struct) => {get::handle(get_struct, config_file).await.expect("Request Error");},
        EntityType::List(list_struct) => {list::handle(list_struct, config_file).await.expect("Request Error");},
        EntityType::Set(set_struct) => {set::handle(set_struct, config_file).await.expect("Request Error");}
        EntityType::Auth(auth_struct) => {auth::handle(auth_struct, config_file).await.expect("Request Error");}
    }
}
