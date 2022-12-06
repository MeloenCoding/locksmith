use clap::Args;
use rand::Rng;

#[derive(Debug, Args)]
pub struct GenCommand {
    /// Length for your password
    pub length: Option<usize>,

    /// Use symbols
    #[arg(short = 's')]
    pub use_symbols: bool
}

pub fn handle_gen(gen_struct: &GenCommand) {
    let mut rng = rand::thread_rng();
    let mut charset: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890");    
    let mut password: String = String::from("");

    let mut length:usize = 12;

    if gen_struct.length.is_some() {        
        length = gen_struct.length.unwrap();
    }

    if gen_struct.use_symbols {
        charset = format!("{}{}", charset, "!@#$%&?");
    }

    for _i in 0..length {
        password.push(charset.chars().nth(rng.gen_range(0..charset.len())).unwrap());
    }

    println!("{}", password);
}
