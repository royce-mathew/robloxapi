mod errors;
mod games;
mod https;
mod users;

pub use errors::ApiResult;
pub use games::models::{DevProduct, Game, Server};
pub use games::GameBuilder;
pub use https::Https;
pub use users::models::User;
pub use users::UserBuilder;

use reqwest::Method;

// STATIC URLS
pub(crate) const BASE: &str = "https://api.roblox.com";
#[allow(dead_code)]
pub(crate) const AUTH: &str = "https://auth.roblox.com/v1/account/pin/unlock";
#[allow(dead_code)]
pub(crate) const ACCOUNT: &str = "https://accountinformation.roblox.com/v1";
#[allow(dead_code)]
pub(crate) const MESSAGES: &str = "https://privatemessages.roblox.com/v1";
pub(crate) const USER: &str = "https://users.roblox.com/v1";
pub(crate) const GAMES: &str = "https://games.roblox.com/v1";
#[allow(dead_code)]
pub(crate) const GROUPS: &str = "https://groups.roblox.com/v1";
#[allow(dead_code)]
pub(crate) const PRESENCE: &str = "https://presence.roblox.com/v1/presence/users";
#[allow(dead_code)]
pub(crate) const ECONOMY: &str = "https://economy.roblox.com/v1/assets";
#[allow(dead_code)]
pub(crate) const INVENTORY: &str = "https://inventory.roblox.com";
#[allow(dead_code)]
pub(crate) const DEVPAGE: &str = "https://develop.roblox.com/v1/universes";

#[derive(Debug, Clone)]
pub struct Client {
    pub session: crate::Https,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Create a new client instance
    pub fn new() -> Self {
        Self {
            session: crate::Https::new(),
        }
    }

    /// Create a new user given user_id
    /// ## Example
    /// ```
    /// use tokio;
    /// use robloxapi;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = robloxapi::Client.new().await;
    ///     let user = client.user(242872495).await;
    /// }
    /// ```
    pub async fn user(&mut self, builder: impl UserBuilder) -> User {
        builder.new(&mut self.session).await
    }

    /// Get the current user. Must be logged in with a cookie to get current_user
    /// # Example
    /// ```
    /// use tokio;
    /// use robloxapi;
    ///
    /// let COOKIE: &str = "";
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = robloxapi::Client.new()
    ///         .set_cookie(COOKIE)
    ///         .await;
    ///     let current_user = client.current_user().await;
    /// }
    //
    pub async fn current_user(self: &mut Client) -> User {
        let data = self
            .session
            .request::<serde_json::Value>(Method::GET, "https://www.roblox.com/mobileapi/userinfo")
            .await
            .expect("Failed to get user info");

        let builder = data.get("UserID").unwrap().as_u64().unwrap();
        UserBuilder::new(builder, &mut self.session).await
    }

    /// Returns a Game struct given the place ID. Get information about a game.
    /// ## Example
    /// ```
    /// use robloxapi;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     
    ///     let place_id = 7415484311; // Place ID for game
    ///     let client = robloxapi::Client() // Initialize a new client instance
    ///         .await;
    ///
    ///     // Create a new game given place id
    ///     let game = client.game(place_id)
    ///         .await;
    /// }
    /// ````
    pub async fn game(&self, builder: impl GameBuilder) -> ApiResult<Game> {
        builder.new(&mut self.session.clone()).await
    }
}
