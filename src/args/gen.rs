use clap::Args;
use rand::Rng;
use crate::auth;
use crate::config::ConfigFile;

#[derive(Debug, Args)]
pub struct GenCommand {
    /// Length for your password
    pub length: Option<usize>,

    /// Use symbols
    #[arg(short = 's')]
    pub use_symbols: bool
}

pub async fn handle(gen_struct: &GenCommand, config_file: ConfigFile) {
    auth::check_auth(&config_file).await.expect("Authentication Error");
    
    let mut length: usize = 12;

    if gen_struct.length.is_some() {
        length = gen_struct.length.unwrap();
    }

    let password = gen_string(length, gen_struct.use_symbols);
    println!("   - [ {} ] -", password);
}

pub fn gen_string(len: usize, use_symbols: bool) -> String{ 
    let mut rng = rand::thread_rng();
    let mut charset: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890");    
    let mut password: String = String::from("");

    if use_symbols {
        charset = format!("{}{}", charset, "!@#$%&?");
    }

    for _i in 0..len {
        password.push(charset.chars().nth(rng.gen_range(0..charset.len())).unwrap());
    }
    return password;
}
