use google_authenticator::GoogleAuthenticator;
use serde::Deserialize;
use crate::args::{config, gen};
use crate::config::ConfigFile;

use clap::Args;
use reqwest::Client;

#[derive(Debug, Args)]
pub struct AuthCommand {
  	/// Name of website 
    pub device_name: String,
    /// Show the qr code to the website that contains the qrcode
    #[arg(short = 'd')]
    pub display_qr: bool,
    /// Shorter secret. Less secure, easier to type over in the google authenticator app.
    #[arg(short)]
    pub short: bool
}
#[derive(Deserialize)]
pub struct AuthCommandReturn {
    data: String
}

pub async fn handle(auth_struct: &AuthCommand, config_file: ConfigFile) -> Result<(), reqwest::Error> {

    let mut len: u8 = 32;

    if auth_struct.short {
        len = 16;
    }

    let auth = google_authenticator::GoogleAuthenticator::new();
    let secret = auth.create_secret(len);

    if auth_struct.display_qr {
        let qr_url = auth.qr_code_url(&secret, "locksmith", &auth_struct.device_name, 200, 200, google_authenticator::ErrorCorrectionLevel::High );
        qr2term::print_qr(qr_url).unwrap(); 
    }

    println!("If you are unable to use the qrcode, you should manualy type this code into your google authenticator app.");
    println!("     - [ {} ] - ", secret);

    let master = gen::gen_string(12, true);

    let client: Client = reqwest::Client::new();
    let _res: AuthCommandReturn = client.post(config_file.location)
		.header("Content-Type", "application/json")
		.json(&serde_json::json!({
			"appId": config_file.app_id,
			"appKey": config_file.app_key,
			"clientKey": config_file.client_key,
			"endpoint": "/save-secret",
			"data": {
                "secret": secret,
                "master": master
            }
		})) 
		.send()
		.await?
		.json()
		.await?;
    Ok(())
}

pub async fn check_auth(config_file: &ConfigFile) -> Result<(), reqwest::Error>{
    print_message();

    let auth = GoogleAuthenticator::new();
    let client = reqwest::Client::new();
    let secret_obj: AuthCommandReturn = client.post("http://localhost:80/overseer")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "appId": config_file.app_id,
			"appKey": config_file.app_key,
			"clientKey": config_file.client_key,
			"endpoint": "/get-secret",
			"data": {}
        }))
        .send()
        .await?
        .json()
        .await?;

    let secret: String = secret_obj.data;

    let mut line = String::new();
    println!("Please enter your 2fa code: ");
    std::io::stdin().read_line(&mut line).unwrap();

    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }

    let code = auth.get_code(&secret, 0).unwrap();
    if line != code {
        config::display_error("2fa code incorrect.".to_string());
    }
    Ok(())
}

fn print_message() {
    println!("
    █    ████▄ ▄█▄    █  █▀  ▄▄▄▄▄   █▀▄▀█ ▄█    ▄▄▄▄▀ ▄  █
    █    █   █ █▀ ▀▄  █▄█   █     ▀▄ █ █ █ ██ ▀▀▀ █   █   █
    █    █   █ █   ▀  █▀▄ ▄  ▀▀▀▀▄   █ ▄ █ ██     █   ██▀▀█   v{} - by {}    
    ███▄ ▀████ █▄  ▄▀ █  █ ▀▄▄▄▄▀    █   █ ▐█    █    █   █        
        ▀      ▀███▀    █               █   ▐   ▀        █
                         ▀               ▀                ▀ ", 
    env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
}
