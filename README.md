# rsblox
rsblox is a async Rust roblox api wrapper; Fork of PythonicIconic's [RbxAPI-rs](https://github.com/PythonicIconic/RbxAPI-rs).

## Usage
TBA

Example of retrieving a given user, three different ways!
```rust
use rbxapi;

#[tokio::main]
async fn main() {
    let cookie = "your_cookie";
    let client = rbxapi::Client.new().cookie(cookie).await;
    
    let my_user = client.current_user().await;
    let str_user = client.user("builderman").await;
    let int_user = client.user(156).await;
}
```