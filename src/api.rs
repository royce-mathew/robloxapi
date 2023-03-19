use reqwest::header;

use crate::{Game, GameBuilder};
use crate::{User, UserBuilder};

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

#[derive(Debug)]
pub struct Client {
    pub session: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            session: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
        }
    }

    pub async fn cookie(mut self, cookie: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&*(".ROBLOSECURITY=".to_owned() + cookie)).unwrap(),
        );
        headers.insert(
            header::CONTENT_LENGTH,
            header::HeaderValue::from_static("0"),
        );

        // Get X-CSRF Token
        let resp = reqwest::Client::new()
            .post("https://catalog.roblox.com/v1/catalog/items/details")
            .header("content-length", "0")
            .send()
            .await
            .expect("Failed to get X-CSRF-TOKEN");

        headers.insert(
            header::HeaderName::from_static("x-csrf-token"),
            header::HeaderValue::from(
                resp.headers()
                    .get("x-csrf-token")
                    .unwrap_or(&header::HeaderValue::from_static("")),
            ),
        );

        // Create a new session with the cookie and token
        self.session = reqwest::Client::builder()
            .cookie_store(true)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.152 Safari/537.36")
            .default_headers(headers)
            .build()
            .expect("Failed to build new client from headers");

        self.validate_cookie().await;
        self
    }

    pub async fn user(&self, builder: impl UserBuilder) -> User {
        builder.new(&self.session).await
    }

    pub async fn current_user(&self) -> User {
        let data = self
            .session
            .get("https://www.roblox.com/mobileapi/userinfo")
            .send()
            .await
            .expect("Failed to get user info")
            .json::<serde_json::Value>()
            .await
            .expect("Failed to get user json");

        let builder = data.get("UserID").unwrap().as_u64().unwrap();
        UserBuilder::new(builder, &self.session).await
    }

    pub async fn game(&self, builder: impl GameBuilder) -> Game {
        builder.new(&self.session).await
    }

    async fn validate_cookie(&self) {
        let resp = self
            .session
            .get("https://www.roblox.com/mobileapi/userinfo")
            .send()
            .await
            .expect("Failed to get user info");
        let _: serde_json::Value = resp.json().await.expect("Failed to get json");
    }
}
