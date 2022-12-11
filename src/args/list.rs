use clap::Args;
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Args)]
pub struct ListCommand {
  	/// Optional page number. Default = 0
  	pub page: Option<usize>
}

#[derive(Debug, Serialize, Deserialize)]
struct ListCommandReturn {
    data: Vec<String>    
}

pub async fn handle_list(list_struct: &ListCommand) -> Result<(), reqwest::Error>{
    let client: Client = reqwest::Client::new();

    let mut page: usize = 0;

    if list_struct.page.is_some() {
        page = list_struct.page.unwrap();
    }

    let res: ListCommandReturn = client.post("http://localhost:80/overseer")
		.header("Content-Type", "application/json")
		.json(&serde_json::json!({
			"appId": "locksmith",
			"appKey": "locksmith123",
			"clientKey": "766145939",
			"endpoint": "/list",
			"data": {
                "page": page 
            }
		})) 
		.send()
		.await?
		.json()
		.await?;

    dbg!(&res);
    Ok(())
}
