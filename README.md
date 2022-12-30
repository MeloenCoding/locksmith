<h1 align="center">
   Locksmith
</h1>
<br>
<p align="center">
  A simple cli tool to generate and store your passwords.
</p>
<br>

## Features
- Google Authenticator (2fa)
- Password Generator
- Remote storage for use on other devices

## Format
```bash
# See all commands
losm -h

# Return all stored websites
losm list

# Generate password 20 characters long and use symbols in it 
losm gen 20 -s

# promt your Google Auth key and returns password of given site
losm get "github" "accountName"

# set password that is or isn't in the database
losm set "github" "accountName" "samplePassword"
```
