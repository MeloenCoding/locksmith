pub mod config;
pub mod gen;
pub mod get;
pub mod list;
pub mod set;
pub mod auth;

use clap:: {
  Parser,
  Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct LocksmithArgs{
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Edit config
    Config(config::ConfigCommand),
    /// Generate password
    Gen(gen::GenCommand),
    /// Get password
    Get(get::GetCommand),
    /// List all names 
    List(list::ListCommand),
    /// Set a password of a given account
    Set(set::SetCommand),
    /// Auth config
    Auth(auth::AuthCommand)
}
