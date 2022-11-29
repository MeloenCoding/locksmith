# Locksmith

A simple cli tool to keep and or generate your passwords.

## Features
- Google Authenticator
- Password Generator
- Local or Remote

## Format
```
# General command
losm

# Return all stored websites
losm list

# Generate password for a website and set it 
losm gen "github"

# promt your Google Auth key and returns password of given site
losm get "github"

# set password that is or isn't in the database
losm set "github" "Sample password"
```
 
