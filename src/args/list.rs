use clap::Args;
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use crate::config::ConfigFile;

#[derive(Debug, Args)]
pub struct ListCommand {
  	/// Optional page number. Default = 0
  	pub page: Option<usize>
}

#[derive(Debug, Serialize, Deserialize)]
struct ListCommandReturn {
    data: Vec<String>    
}

pub async fn handle_list(list_struct: &ListCommand, config_file: ConfigFile) -> Result<(), reqwest::Error>{
    let client: Client = reqwest::Client::new();

    let mut page: usize = 0;

    if list_struct.page.is_some() {
        page = list_struct.page.unwrap();
    }

    let res: ListCommandReturn = client.post("http://localhost:80/overseer")
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

    println!("Page {}", page);
    let _i: i32 = 0;
    for site in res.data {
        // if i > page * 5 {
            println!(" - {}", site);
        // }
    }
    Ok(())
}
