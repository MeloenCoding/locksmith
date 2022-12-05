use clap::Args;

#[derive(Debug, Args)]
pub struct SetCommand {
  	/// Name of website or account. Whatever you prefer
  	pub account: String,
	pub new_password: String
}

pub fn handle_set(set_struct: &SetCommand) {
	dbg!(set_struct);
    
}
