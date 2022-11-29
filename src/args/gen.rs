use clap::Args;

#[derive(Debug, Args)]
pub struct GenCommand {
  	/// Name of website or account. Whatever you prefer
  	pub account: String,
	pub new_password: String
}