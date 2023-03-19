use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::error;
use std::collections::HashMap;

#[async_trait]
pub trait GameBuilder {
    async fn new(self, client: &reqwest::Client) -> Game;
}

#[async_trait]
impl GameBuilder for u64 {
    async fn new(self, client: &reqwest::Client) -> Game {
        let data = client
            .get(&format!(
                "{}/games/multiget-place-details?placeIds={}",
                crate::api::GAMES,
                self
            ))
            .send()
            .await
            .expect("Failed to get game universe info")
            .json::<serde_json::Value>()
            .await
            .expect("Failed to get game universe json");

        let data = client
            .get(&format!(
                "{}/games?universeIds={}",
                crate::api::GAMES,
                data[0]
                    .get("universeId")
                    .expect("Failed to find game universe ID")
            ))
            .send()
            .await
            .expect("Failed to get game root info")
            .json::<serde_json::Value>()
            .await
            .expect("Failed to get game root json");

        Game {
            auth: client.clone(),
            ..serde_json::from_value(
                data.get("data").expect("Failed to get game root data")[0].clone(),
            )
            .expect("Failed to parse into Game")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub id: String,
    #[serde(rename = "maxPlayers")]
    pub max_players: u8,
    pub playing: u32,
    pub fps: f32,
    pub ping: u32,
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Game(id={}, playing={}, max_players={})",
            self.id, self.playing, self.max_players
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    #[serde(skip)]
    auth: reqwest::Client,
    #[serde(skip)]
    servers: Option<Vec<Server>>,

    #[serde(rename = "id")]
    pub universe_id: u64,
    #[serde(rename = "rootPlaceId")]
    pub place_id: u64,
    pub name: String,
    pub description: String,
    pub price: Option<u64>,
    #[serde(rename = "allowedGearGenres")]
    pub allowed_gear_genres: Vec<String>,
    #[serde(rename = "allowedGearCategories")]
    pub allowed_gear_categories: Vec<String>,
    pub playing: u32,
    pub visits: u64,
    #[serde(rename = "maxPlayers")]
    pub max_players: u8,
    pub created: String,
    pub updated: String,
    #[serde(rename = "studioAccessToApisAllowed")]
    pub studio_access_to_apis_allowed: bool,
    #[serde(rename = "createVipServersAllowed")]
    pub create_vip_servers_allowed: bool,
    #[serde(rename = "universeAvatarType")]
    pub universe_avatar_type: String,
    pub genre: String,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Game(placeid={}, name={})", self.place_id, self.name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DevProduct {
    pub name: String,
    #[serde(skip)]
    pub price: u32,
    pub id: u64,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "iconImageAssetId")]
    pub image_asset_id: Option<u64>,
    #[serde(rename = "shopId")]
    pub shop_id: u64,
}

impl std::fmt::Display for DevProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "DevProduct(name={}, price={}, product_id={})",
            self.name, self.price, self.id
        )
    }
}

impl Game {
    pub async fn servers(&mut self) -> Vec<Server> {
        if let Some(servers) = self.servers.clone() {
            return servers;
        } else {
            let mut servers: Vec<Server> = vec![];
            let mut data = self
                .auth
                .get(&format!(
                    "{}/games/{}/servers/Public?limit=100",
                    crate::api::GAMES,
                    self.place_id
                ))
                .send()
                .await
                .expect("Failed to get server list")
                .json::<serde_json::Value>()
                .await
                .expect("Failed to get server json");

            while let Some(cursor) = data.clone().get("nextPageCursor") {
                if cursor.is_null() {
                    break;
                }

                if let Some(info) = data.get("data") {
                    let data_servers: Vec<Server> =
                        serde_json::from_value(info.clone()).unwrap_or(vec![]);
                    servers.extend_from_slice(&data_servers[..]);
                }

                data = self
                    .auth
                    .get(&format!(
                        "{}/games/{}/servers/Public?limit=100&cursor={}",
                        crate::api::GAMES,
                        self.place_id,
                        cursor.as_str().unwrap()
                    ))
                    .send()
                    .await
                    .expect("Failed to get server list")
                    .json::<serde_json::Value>()
                    .await
                    .expect("Failed to get server json");

                if let Some(error) = data.get("errors") {
                    if let Some(message) = error[0].get("message") {
                        if message.as_str().unwrap() == "TooManyRequests" {
                            println!("Rate limited, sleeping for 3 seconds");
                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

                            data = self
                                .auth
                                .get(&format!(
                                    "{}/games/{}/servers/Public?limit=100&cursor={}",
                                    crate::api::GAMES,
                                    self.place_id,
                                    cursor.as_str().unwrap()
                                ))
                                .send()
                                .await
                                .expect("Failed to get server list")
                                .json::<serde_json::Value>()
                                .await
                                .expect("Failed to get server json");
                        }
                    }

                    continue;
                }
            }

            self.servers = Some(servers.clone());
            servers
        }
    }

    pub async fn create_dev_product(&self, name: String, price: u32) -> DevProduct {
        // Get X-CSRF-TOKEN
        let auth_resp = self
            .auth
            .post("https://catalog.roblox.com/v1/catalog/items/details")
            .header("content-length", "0")
            .send()
            .await
            .expect("Failed to get X-CSRF-TOKEN");

        // Make Request To DeveloperProducts
        let data = self
            .auth
            .post(&format!(
                "{}/{}/developerproducts?name={}&description={}&priceInRobux={}",
                crate::api::DEVPAGE,
                self.universe_id,
                name,
                price,
                price
            ))
            .header(
                "x-csrf-token",
                auth_resp.headers().get("x-csrf-token").unwrap(),
            )
            .header("content-length", "0")
            .send()
            .await
            .expect("Failed to create dev product");

        // Get json Data
        let json_data = data
            .json::<serde_json::Value>()
            .await
            .expect("Failed to get dev product json");

        // Check if any errrs occurred
        if let Some(error) = json_data.get("errors") {
            if error[0]["code"] == 4 {
                // Developer Product Already exists

                // Get the developer product internally here?
                println!("Product already exists, retrying");
            } else {
                panic!("Err: {}", error[0]);
            }
        }

        let product = DevProduct {
            price,
            ..serde_json::from_value(json_data).expect("Failed to parse into DevProduct")
        };
        product
    }

    // pub async fn get_dev_product(&self, name: String) -> DevProduct {
    //     // TODO
    // }
}
