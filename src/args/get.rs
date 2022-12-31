use clap::Args;
use serde::Deserialize;
use crate::{auth, config::ConfigFile, args::config};
use reqwest::Client;

#[derive(Debug, Args)]
pub struct GetCommand {
  	/// Name of website or account. Whatever you prefer
    pub location: String,
  	pub account: String
}

#[derive(Debug, Deserialize)]
pub struct GetCommandReturn {
    valid: bool,
    data: Option<String>
}

pub async fn handle(get_struct: &GetCommand, config_file: ConfigFile) -> Result<(), reqwest::Error> {
    if auth::check_auth(&config_file).await.is_err() {
		config::display_error("Authentication server not reachable".into());
	}

    let client: Client = reqwest::Client::new();

    let res: GetCommandReturn = client.post(config_file.location)
		.header("Content-Type", "application/json")
		.json(&serde_json::json!({
			"appId": config_file.app_id,
			"appKey": config_file.app_key,
			"clientKey": config_file.client_key,
			"endpoint": "/get",
			"data": {
                "location": get_struct.location,
                "account": get_struct.account 
            }
		})) 
		.send()
		.await?
		.json()
		.await?;
	
    let password: String = res.data.unwrap_or("not-found".to_string());

    if password != "not-found" {
        println!("Your password for {} is [ {} ]", get_struct.location, password);
        return Ok(());
    }

    Ok(config::display_error("Invalid location or account.".into()))
}

