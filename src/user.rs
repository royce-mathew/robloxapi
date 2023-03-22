use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait UserBuilder {
    async fn new(self, client: &mut crate::Https) -> User;
}

#[async_trait]
impl UserBuilder for &str {
    /// Create a new user by name
    async fn new(self, client: &mut crate::Https) -> User {
        let mut map = HashMap::new();
        map.insert("usernames", vec![self]);

        let data = client
            .post(&format!("{}/usernames/users", crate::api::USER))
            .await
            .json(&map)
            .header("content-length", serde_json::to_vec(&map).unwrap().len())
            .send()
            .await
            .expect("Failed to get user info")
            .json::<serde_json::Value>()
            .await
            .expect("Failed to get user json");

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
    /// Create a new user with userid
    async fn new(self, client: &mut crate::Https) -> User {
        let user: User = client.client
            .request(Method::GET, &format!("{}/users/{}", crate::api::BASE, self))
            .send()
            .await
            .expect("Failed to get user info from base")
            .json()
            .await
            .expect("Failed to update struct with base");

        let mut u2 = User {
            id: Some(self),
            username: user.username,
            avatarfinal: user.avatarfinal,
            avataruri: user.avataruri,
            isonline: user.isonline,
            ..client.client
                .request(Method::GET, &format!("{}/users/{}", crate::api::USER, self))
                .send()
                .await
                .expect("Failed to get user info from user")
                .json()
                .await
                .expect("Failed to update struct with user")
        };

        u2.client = Some(client.clone());
        u2
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(skip)]
    client: Option<crate::Https>,
    #[serde(skip)]
    friends: Option<Vec<User>>,

    #[serde(rename = "Id")]
    pub id: Option<u64>,
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "AvatarFinal")]
    pub avatarfinal: Option<bool>,
    #[serde(rename = "AvatarUri")]
    pub avataruri: Option<String>,
    pub created: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "isBanned")]
    pub isbanned: Option<bool>,
    #[serde(rename = "IsOnline")]
    pub isonline: Option<bool>,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let unknown = String::from("unknown");
        write!(
            f,
            "User(id={}, username={}, created={}, isonline={}, isbanned={})",
            self.id.as_ref().unwrap(),
            self.username.as_ref().unwrap_or(&unknown),
            self.created.as_ref().unwrap_or(&unknown),
            self.isonline.as_ref().unwrap_or(&false),
            self.isbanned.as_ref().unwrap_or(&false)
        )
    }
}

impl User {
    /// Get all friends of user, requires cookie
    pub async fn friends(&mut self) -> Vec<User> {
        if let Some(friends) = self.friends.clone() {
            friends
        } else {
            let mut friends: Vec<User> = vec![];
            let mut page: i32 = 1;
            let mut page_string = format!("?page={}", page);

            let mut data = self
                .client
                .as_mut()
                .unwrap()
                .client
                .request(Method::GET, &format!(
                    "{}/users/{}/friends{}",
                    crate::api::BASE,
                    self.id.unwrap(),
                    page_string
                ))
                .send()
                .await
                .expect("Failed to get friends list")
                .json::<Vec<User>>()
                .await
                .expect("Failed to get friends json");

            loop {
                friends.extend_from_slice(&data[..]);

                page += 1;
                page_string = format!("?page={}", page);
                data = self
                    .client
                    .as_mut()
                    .unwrap()
                    .client
                    .request(Method::GET, &format!(
                        "{}/users/{}/friends{}",
                        crate::api::BASE,
                        self.id.unwrap(),
                        page_string
                    ))
                    .send()
                    .await
                    .expect("Failed to get friends list")
                    .json::<Vec<User>>()
                    .await
                    .expect("Failed to get friends json");

                if data.is_empty() {
                    break;
                }
            }

            self.friends = Some(friends.clone());
            friends
        }
    }

    /// Check if user has asset, may require cookie
    pub async fn has_asset(&mut self, asset_id: u64) -> bool {
        self.client.as_mut().unwrap().client
            .request(Method::GET, &format!(
                "{}/ownership/hasasset?userId={}&assetId={}",
                crate::api::BASE,
                self.id.unwrap(),
                asset_id
            ))
            .send()
            .await
            .expect("Failed to get ownership info")
            .json::<bool>()
            .await
            .expect("Failed to get ownership json")
    }
}
