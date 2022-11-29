use clap::Args;

#[derive(Debug, Args)]
pub struct ListCommand {
  	/// Name of website or account. Whatever you prefer
  	pub page: i8
}

pub fn handle_set(set_struct: &ListCommand) {
	dbg!(set_struct);
}