<h1 align="center">
   Locksmith
</h1>
<br>
<p align="center">
  A simple cli tool to generate and store your passwords.
</p>
<br>

## Features
- A master password for all your saved passwords
- Google Authenticator (2fa)
- Password Generator
- Remote storage for use on other devices
- Local for decentralized

## How it works
### Remote
Passwords that you generate will be generated localy. So it won't fly over the internet. 

Before getting or setting a password you'll need 

The password and the name of the website or service will get encrypted with the encryption method of your choice. Then it wil send it to the server. This prevents your password being send over the internet without encryption.

### Local
Everything will be generated and stored localy. It will be encrypted by the encryption method of you choice. 

## Format
```bash
# See all commands
losm -h

# Set personal encryption key
losm config key "random personal key"

# Return all stored websites
losm list

# Generate password for a website and set it 
losm gen "github"

# promt your Google Auth key and returns password of given site
losm get "github"

# set password that is or isn't in the database
losm set "github" "Sample password"
```
