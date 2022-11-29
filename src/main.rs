use clap::Parser;
// use directories::ProjectDirs;

mod args;

use args::{set, list, EntityType};

use crate::args::{config, gen, get};

fn main() {
    println!("Hello, world!");

    let args: args::LocksmithArgs = args::LocksmithArgs::parse();
    
    match &args.entity_type {
        EntityType::Config(config_struct) => config::handle_config(config_struct),
        EntityType::Gen(gen_struct) => gen::handle_gen(gen_struct),
        EntityType::Get(get_struct) => get::handle_get(get_struct),
        EntityType::List(list_struct) => list::handle_set(list_struct),
        EntityType::Set(set_struct) => set::handle_set(set_struct)
    }
}
