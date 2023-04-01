# RobloxApi
[![Latest Version](https://img.shields.io/crates/v/robloxapi.svg)](https://crates.io/crates/robloxapi) [![Docs](https://img.shields.io/badge/docs.rs-robloxapi-green)](https://docs.rs/robloxapi)

`robloxapi` is a open source async Rust API wrapper for roblox; Fork of PythonicIconic's [RbxAPI-rs](https://github.com/PythonicIconic/RbxAPI-rs). 

# Getting Started
You can install the library by running `cargo add robloxapi`

### Retrieving Users
Example of retrieving a given user, three different ways!
```rust
use robloxapi;
use tokio;

// The cookie is needed for several api endpoints; Specifically those which interact with acccount / game data.
const COOKIE: &str = ""

#[tokio::main]
async fn main() {
    let mut client = rbxapi::Client.new(); // Create new client Instance
    client.set_cookie(COOKIE).await; // Set the cookie for the client instance
    
    // Example on getting users
    let my_user = client.current_user().await?; // Get the current user
    let str_user = client.user("builderman").await?; // Get user by username
    let int_user = client.user(156).await?; // Get user by userid
}
```

### Developer Products / Games
```rust
use robloxapi;
use tokio;

const COOKIE: &str = ""

#[tokio::main]
async fn main() {
   let place_id = 7415484311; // Place ID for game
   let mut client = robloxapi::Client(); // Create a new client instance
   client.set_cookie(COOKIE).await; // We need to set the cookie if we want to have permissions for creating developer products

   // Create a new game given place id
   let mut game = client.game(place_id).await?;

    // Fails if a devproduct already exists with the name
    let dev_product = game.create_dev_product(
        "name-of-dev-product", // Name of the developer product
        17 // Price of the developer product
    ).await?;
}

```
