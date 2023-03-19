use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait GameBuilder {
    async fn new(self, client: &reqwest::Client) -> Game;
}

#[async_trait]
impl GameBuilder for u64 {
    async fn new(self, client: &reqwest::Client) -> Game {
        let data = client.get(&format!("{}/games/multiget-place-details?placeIds={}", crate::api::GAMES, self))
            .send().await.expect("Failed to get game universe info")
            .json::<serde_json::Value>().await.expect("Failed to get game universe json");

        let data = client.get(&format!("{}/games?universeIds={}", crate::api::GAMES, data[0].get("universeId").expect("Failed to find game universe ID")))
                .send().await.expect("Failed to get game root info")
                .json::<serde_json::Value>().await.expect("Failed to get game root json");

        Game {
            auth: client.clone(),
            ..serde_json::from_value(
                    data.get("data")
                    .expect("Failed to get game root data")
                    [0].clone()
                )
                .expect("Failed to parse into Game")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub id: String,
    #[serde(rename="maxPlayers")]
    pub max_players: u8,
    pub playing: u32,
    pub fps: f32,
    pub ping: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    #[serde(skip)]
    auth: reqwest::Client,
    #[serde(skip)]
    servers: Option<Vec<Server>>,
    #[serde(skip)]
    dev_products: Option<HashMap<String, DevProduct>>,

    #[serde(rename="id")]
    pub universe_id: u64,
    #[serde(rename="rootPlaceId")]
    pub place_id: u64,
    pub name: String,
    pub description: String,
    pub price: Option<u64>,
    #[serde(rename="allowedGearGenres")]
    pub allowed_gear_genres: Vec<String>,
    #[serde(rename="allowedGearCategories")]
    pub allowed_gear_categories: Vec<String>,
    pub playing: u32,
    pub visits: u64,
    #[serde(rename="maxPlayers")]
    pub max_players: u8,
    pub created: String,
    pub updated: String,
    #[serde(rename="studioAccessToApisAllowed")]
    pub studio_access_to_apis_allowed: bool,
    #[serde(rename="createVipServersAllowed")]
    pub create_vip_servers_allowed: bool,
    #[serde(rename="universeAvatarType")]
    pub universe_avatar_type: String,
    pub genre: String
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Game(placeid={}, name={})", self.place_id, self.name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DevProduct {
    pub name: String,
    pub price: u32,
    pub id: u64,
}

impl std::fmt::Display for DevProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DevProduct(name={}, price={}, product_id={})", self.name, self.price, self.id)
    }
}


impl Game {
    pub async fn servers(&mut self) -> Vec<Server> {
        if let Some(servers) = self.servers.clone() {
            return servers;
        } else {
            let mut servers: Vec<Server> = vec![];
            let mut data = self.auth.get(&format!("{}/games/{}/servers/Public?limit=100", crate::api::GAMES, self.place_id))
                .send().await.expect("Failed to get server list")
                .json::<serde_json::Value>().await.expect("Failed to get server json");

            while let Some(cursor) = data.clone().get("nextPageCursor") {
                if cursor.is_null() { break }

                if let Some(info) = data.get("data") {
                    let data_servers: Vec<Server> = serde_json::from_value(info.clone()).unwrap_or(vec![]);
                    servers.extend_from_slice(&data_servers[..]);
                }

                data = self.auth.get(&format!("{}/games/{}/servers/Public?limit=100&cursor={}", crate::api::GAMES, self.place_id, cursor.as_str().unwrap()))
                    .send().await.expect("Failed to get server list")
                    .json::<serde_json::Value>().await.expect("Failed to get server json");

                if let Some(error) = data.get("errors") {
                    if let Some(message) = error[0].get("message") {
                        if message.as_str().unwrap() == "TooManyRequests" {
                            println!("Rate limited, sleeping for 3 seconds");
                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

                            data = self.auth.get(&format!("{}/games/{}/servers/Public?limit=100&cursor={}", crate::api::GAMES, self.place_id, cursor.as_str().unwrap()))
                                .send().await.expect("Failed to get server list")
                                .json::<serde_json::Value>().await.expect("Failed to get server json");
                        }
                    }

                    continue
                }
            }

            self.servers = Some(servers.clone());
            servers
        }
    }

    pub async fn create_dev_product(&mut self, name: String, price: u32) -> DevProduct {
        if let Some(dev_products) = self.dev_products.clone() {
            dev_products[&name].clone()
        } else {
            let mut dev_products: HashMap<String, DevProduct> = HashMap::new();

            let data = self.auth.post(
                    format!("{}/{}/developerproducts?name={}&priceInRobux={}", crate::api::DEVPAGE, self.universe_id, name, price)
                )
                .send()
                .await
                .expect("Failed to create dev product")
                .json::<serde_json::Value>()
                .await
                .expect("Failed to get dev product json");

            println!("{:?}", data);

            let product = DevProduct {
                ..serde_json::from_value(
                    data.get("data")
                    .expect("Failed to get game root data")
                    [0].clone()
                )
                .expect("Failed to parse into Game")
            };

            dev_products.insert(name, product.clone());
            self.dev_products = Some(dev_products.clone());
            product
        }
    }
}
