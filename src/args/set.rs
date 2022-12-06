use clap::Args;
use reqwest;
use serde_json::{Result, Value};

#[derive(Debug, Args)]
pub struct SetCommand {
  	/// Name of website or account. Whatever you prefer
  	pub account: String,
	pub new_password: String
}

pub async fn handle_set(set_struct: &SetCommand) -> Result<(), reqwest::Error> {
	dbg!(set_struct);
    let client = reqwest::Client::new();
	
	let data = r#"
	{
		"appId": "locksmith", 
		"appKey": "locksmith123", 
		"clientKey": "766145939",
		"data": {
		  "settings" : {
			"noSymbols" : false 
		  }
		},
		"endpoint": "/gen"
	}"#;

	let res = client.post("http://localhost:80/overseer")
		.header("Content-Type", "application/json")
		.json(data)
		.send()
		.await?
		.json()
		.await?;
}
