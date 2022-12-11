use clap::Args;

#[derive(Debug, Args)]
pub struct AuthCommand {
    pub set: String
}

pub fn handle_auth(auth_struct: &AuthCommand) {
    println!("{:?}", auth_struct);
}

pub fn check_auth() {
   println!("valid"); 
}
