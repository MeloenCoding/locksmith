use clap::Args;
use reqwest::{self, Client};
use serde::Deserialize;
use crate::{auth, config::ConfigFile};

#[derive(Debug, Args)]
pub struct SetCommand {
  	/// Name of website 
    pub location: String,
    /// Name of your account
  	pub name: String,
	/// The password you want to add
	pub new_password: String
}

#[derive(Debug, Deserialize)]
struct SetCommandReturn {
	_data: String
}

pub async fn handle(set_struct: &SetCommand, config_file: ConfigFile) -> Result<(), reqwest::Error>{
    auth::check_auth(&config_file).await.expect("Authentication Error");
    let client: Client = reqwest::Client::new();

	let _res: SetCommandReturn = client.post(config_file.location)
		.header("Content-Type", "application/json")
		.json(&serde_json::json!({
			"appId": config_file.app_id,
			"appKey": config_file.app_key,
			"clientKey": config_file.client_key,
			"endpoint": "/set",
			"data": {
				"hash": set_struct.new_password,
				"location": set_struct.location,
                "account": set_struct.name
			}
		})) 
		.send()
		.await?
		.json()
		.await?;
    println!("Password saved");
    Ok(())
}
