use clap::Args;
use serde::Deserialize;
use crate::{auth, config::ConfigFile};
use reqwest::Client;

#[derive(Debug, Args)]
pub struct GetCommand {
  	/// Name of website or account. Whatever you prefer
    pub location: String,
  	pub account: String
}

#[derive(Debug, Deserialize)]
pub struct GetCommandReturn {
    data: String
}

pub async fn handle(get_struct: &GetCommand, config_file: ConfigFile) -> Result<(), reqwest::Error> {
    auth::check_auth(&config_file).await.expect("Authentication Error");
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
		
    if res.data != "not-found" {
        dbg!(res.data);
        return Ok(());
    }
    
    println!("Invalid location or account.");
    return Ok(());
}

