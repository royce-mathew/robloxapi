use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait GameBuilder {
    async fn new(self, client: &mut crate::Https) -> Game;
}

#[async_trait]
impl GameBuilder for u64 {
    async fn new(self: u64, client: &mut crate::Https) -> Game {
        let data: _ = client
            .request(Method::GET,
                &format!(
                    "{}/games/multiget-place-details?placeIds={}",
                    crate::api::GAMES,
                    self
                )
            )
            .await
            .expect("Failed to get game universe info");

        let data = client
            .request(Method::GET, &format!(
                "{}/games?universeIds={}",
                crate::api::GAMES,
                data[0]
                    .get("universeId")
                    .expect("Failed to find game universe ID")
            ))
            .await
            .expect("Failed to get game root info");

        Game {
            client: client.clone(),
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
    client: crate::Https,
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
    /// Get a list of servers from the Game
    /// # Example
    /// ```
    /// use robloxapi;
    /// use tokio;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     
    ///     // Place ID
    ///     let game_id = 7415484311;
    ///     let mut client = robloxapi::Client()
    ///         .await;
    ///     // List of servers
    ///     let servers = client.game(game_id)
    ///         .await
    ///         .servers();
    /// }
    /// 
    /// ```
    pub async fn servers(&mut self) -> Vec<Server> {
        if let Some(servers) = self.servers.clone() {
            servers
        } else {
            let mut servers: Vec<Server> = vec![];
            let mut data = self
                .client
                .request(Method::GET, &format!(
                    "{}/games/{}/servers/Public?limit=100",
                    crate::api::GAMES,
                    self.place_id
                ))
                .await
                .expect("Failed to get server list");

            while let Some(cursor) = data.clone().get("nextPageCursor") {
                if cursor.is_null() {
                    break;
                }

                if let Some(info) = data.get("data") {
                    let data_servers: Vec<Server> =
                        serde_json::from_value(info.clone()).unwrap_or_default();
                    servers.extend_from_slice(&data_servers[..]);
                }

                data = self
                    .client
                    .request(Method::GET, &format!(
                        "{}/games/{}/servers/Public?limit=100&cursor={}",
                        crate::api::GAMES,
                        self.place_id,
                        cursor.as_str().unwrap()
                    ))
                    .await
                    .expect("Failed to get server list");

                if let Some(error) = data.get("errors") {
                    if let Some(message) = error[0].get("message") {
                        if message.as_str().unwrap() == "TooManyRequests" {
                            println!("Rate limited, sleeping for 3 seconds");
                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

                            data = self
                                .client
                                .request(Method::GET,&format!(
                                    "{}/games/{}/servers/Public?limit=100&cursor={}",
                                    crate::api::GAMES,
                                    self.place_id,
                                    cursor.as_str().unwrap()
                                ))
                                .await
                                .expect("Failed to get server list");
                        }
                    }

                    continue;
                }
            }

            self.servers = Some(servers.clone());
            servers
        }
    }

    /// Create a developer product given name and price.
    /// # Example
    /// ```
    /// // Create a mew game with place id 100000
    /// let game = client.game(100000).await;
    ///  // Requires client to be logged in with a cookie
    /// let dev_product = game.create_dev_product(
    ///     "devproduct1", // Name of the devproduct
    ///     500, // Price of the devproduct
    /// )
    /// ```
    /// 
    pub async fn create_dev_product(&mut self, name: &str, price: u32) -> DevProduct {
        // Make Request To DeveloperProducts
        let data = self
            .client
            .request(Method::POST, &format!(
                "{}/{}/developerproducts?name={}&description={}&priceInRobux={}",
                crate::api::DEVPAGE,
                self.universe_id,
                name,
                price,
                price
            ))
            .await
            .expect("Failed to create dev product");

        // Check if any errrs occurred
        if let Some(error) = data.get("errors") {
            if error[0]["code"] == 4 {
                // Developer Product Already exists
                // Get the developer product internally here?
                panic!("Product already exists");
            } else {
                panic!("Err: {}", error[0]);
            }
        }

        DevProduct {
            price,
            ..serde_json::from_value(data).expect("Failed to parse into DevProduct")
        }
    }
}
