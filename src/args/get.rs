use clap::Args;

#[derive(Debug, Args)]
pub struct GetCommand {
  	/// Name of website or account. Whatever you prefer
  	pub account: String,
	pub new_password: String
}