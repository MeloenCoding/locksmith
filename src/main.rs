use clap::Parser;
// use directories::ProjectDirs;

mod args;

use args::EntityType;

fn main() {
    println!("Hello, world!");

    let args: args::LocksmithArgs = args::LocksmithArgs::parse();
    match &args.entity_type {
        EntityType::Config(config_command) => {
            dbg!(config_command);
        },
        EntityType::Gen(gen_command) => {
            dbg!(gen_command);
        },
        EntityType::Get(get_command) => {
            dbg!(get_command);
        },
        EntityType::List(list_command) => {
            dbg!(list_command);
        },
        EntityType::Set(set_command) => {
            dbg!(set_command);
        }
    }
}
