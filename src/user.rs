use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use async_trait::async_trait;

#[async_trait]
pub trait UserBuilder {
    async fn new(self, client: &reqwest::Client) -> User;
}

#[async_trait]
impl UserBuilder for &str {
    async fn new(self, client: &reqwest::Client) -> User {
        let mut map = HashMap::new();
        map.insert("usernames", vec![self]);

        let data = client.post(&format!("{}/usernames/users", crate::api::USER))
            .json(&map).header("content-length", serde_json::to_vec(&map).unwrap().len())
            .send().await.expect("Failed to get user info")
            .json::<serde_json::Value>().await.expect("Failed to get user json");

        if let Some(users) = data.get("data") {
            if let Some(id) = users[0].get("id") {
                let builder = id.as_i64().unwrap() as u64;
                builder.new(client).await
            } else {
                panic!("Failed to find users with given name")
            }
        } else {
            panic!("Request failed")
        }
    }
}

#[async_trait]
impl UserBuilder for u64 {
    async fn new(self, client: &reqwest::Client) -> User {
        let user: User = client.get( &format!("{}/users/{}", crate::api::BASE, self))
            .send().await.expect("Failed to get user info from base")
            .json().await.expect("Failed to update struct with base");

        let mut u2 = User {
            id: Some(self),
            username: user.username,
            avatarfinal: user.avatarfinal,
            avataruri: user.avataruri,
            isonline: user.isonline,
            ..client.get(&format!("{}/users/{}", crate::api::USER, self))
                .send().await.expect("Failed to get user info from user")
                .json().await.expect("Failed to update struct with user")
        };

        u2.auth = Some(client.clone());
        u2
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(skip)]
    auth: Option<reqwest::Client>,
    #[serde(skip)]
    friends: Option<Vec<User>>,

    #[serde(rename="Id")]
    pub id: Option<u64>,
    #[serde(rename="Username")]
    pub username: Option<String>,
    #[serde(rename="AvatarFinal")]
    pub avatarfinal: Option<bool>,
    #[serde(rename="AvatarUri")]
    pub avataruri: Option<String>,
    pub created: Option<String>,
    pub description: Option<String>,
    #[serde(rename="isBanned")]
    pub isbanned: Option<bool>,
    #[serde(rename="IsOnline")]
    pub isonline: Option<bool>
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let unknown = String::from("unknown");
        write!(f, "User(id={}, username={}, created={}, isonline={}, isbanned={})",
               self.id.as_ref().unwrap(), self.username.as_ref().unwrap_or(&unknown),
               self.created.as_ref().unwrap_or(&unknown), self.isonline.as_ref().unwrap_or(&false),
               self.isbanned.as_ref().unwrap_or(&false))
    }
}

impl User {
    pub async fn friends(&mut self) -> Vec<User> {
        if let Some(friends) = self.friends.clone() {
            return friends;
        } else {
            let mut friends: Vec<User> = vec![];
            let mut page: i32 = 1;
            let mut page_string = format!("?page={}", page);

            let mut data = self.auth.as_ref().unwrap().get(&format!("{}/users/{}/friends{}", crate::api::BASE, self.id.unwrap(), page_string))
                .send().await.expect("Failed to get friends list")
                .json::<Vec<User>>().await.expect("Failed to get friends json");

            loop {
                friends.extend_from_slice(&data[..]);

                page += 1;
                page_string = format!("?page={}", page);
                data = self.auth.as_ref().unwrap().get(&format!("{}/users/{}/friends{}", crate::api::BASE, self.id.unwrap(), page_string))
                    .send().await.expect("Failed to get friends list")
                    .json::<Vec<User>>().await.expect("Failed to get friends json");

                if data.is_empty() { break }
            }

            self.friends = Some(friends.clone());
            friends
        }
    }

    pub async fn has_asset(&self, asset_id: u64) -> bool {
        self.auth.as_ref().unwrap_or(&reqwest::Client::new()).get(&format!("{}/ownership/hasasset?userId={}&assetId={}", crate::api::BASE, self.id.unwrap(), asset_id))
            .send().await.expect("Failed to get ownership info")
            .json::<bool>().await.expect("Failed to get ownership json")
    }
}
