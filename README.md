# RobloxApi
[![Latest Version](https://img.shields.io/crates/v/robloxapi.svg)](https://crates.io/crates/robloxapi) [![Docs](https://img.shields.io/badge/docs.rs-robloxapi-green)](https://docs.rs/robloxapi)

`robloxapi` is a open source async Rust API wrapper for roblox; Fork of PythonicIconic's [RbxAPI-rs](https://github.com/PythonicIconic/RbxAPI-rs). 

# Getting Started
You can install the library by running `cargo add roboxapi`

### Retrieving Users
Example of retrieving a given user, three different ways!
```rust
use rbxapi;

#[tokio::main]
async fn main() {
    let cookie = "your_cookie";
    let mut client = rbxapi::Client.new().cookie(cookie).await;
    
    let my_user = client.current_user().await?;
    let str_user = client.user("builderman").await?;
    let int_user = client.user(156).await?;
}
```

### Developer Products / Games
```rust
use robloxapi;
use tokio;

#[tokio::main]
async fn main() {
   let place_id = 7415484311; // Place ID for game
   let mut client = robloxapi::Client() // Initialize a new client instance
      .await;

   // Create a new game given place id
   let game = client.game(place_id).await?;

    // Fails if a devproduct already exists with the name
    let dev_product = game.create_dev_product("name-of-dev-product", 17).await?;
}

```
