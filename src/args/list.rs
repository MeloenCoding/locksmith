use clap::Args;
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use crate::{auth, config::ConfigFile, args::config};

#[derive(Debug, Args)]
pub struct ListCommand {
  	/// Optional page number. Default = 0
  	pub page: Option<usize>
}

#[derive(Debug, Serialize, Deserialize)]
struct ListCommandReturn {
    valid: bool,
    data: Option<Vec<String>>    
}

pub async fn handle(list_struct: &ListCommand, config_file: ConfigFile) -> Result<(), reqwest::Error>{
    if auth::check_auth(&config_file).await.is_err() {
		config::display_error("Authentication server not reachable".into());
	}
    let client: Client = reqwest::Client::new();

    let mut page: usize = 0;

    if list_struct.page.is_some() {
        page = list_struct.page.unwrap();
    }

    let res: ListCommandReturn = client.post(config_file.location)
		.header("Content-Type", "application/json")
		.json(&serde_json::json!({
			"appId": config_file.app_id,
			"appKey": config_file.app_key,
			"clientKey": config_file.client_key,
			"endpoint": "/list",
			"data": {
                "page": page 
            }
		})) 
		.send()
		.await?
		.json()
		.await?;
    
    if res.data.is_none() | !res.valid {
        config::display_error("Invalid response from server while fetching list".to_string());
    }
    
    println!("Page {}", page);
    for site in res.data.unwrap() {
        println!(" - {}", site);
    }
    Ok(())

}
