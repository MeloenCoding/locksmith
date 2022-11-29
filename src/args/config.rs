use clap:: {
  Args,
  Subcommand
};

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

pub fn handle_config(conf_struct: &ConfigCommand) {
  dbg!(conf_struct);
}