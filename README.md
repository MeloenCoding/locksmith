<h1 align="center">
   Locksmith
</h1>
<br>
<p align="center">
  A simple cli tool to generate and store your passwords.
</p>
<br>

## Features
- Google Authenticator
- Password Generator
- Local or Remote

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
 
