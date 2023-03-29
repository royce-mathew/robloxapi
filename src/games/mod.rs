pub mod models;

use self::models::{DevProduct, Game, Server};
use crate::ApiResult;

use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
pub trait GameBuilder {
    async fn new(self, client: &mut crate::Https) -> ApiResult<Game>;
}

#[async_trait]
impl GameBuilder for u64 {
    async fn new(self: u64, client: &mut crate::Https) -> ApiResult<Game> {
        let udata: _ = client
            .request::<serde_json::Value>(
                Method::GET,
                &format!(
                    "{}/games/multiget-place-details?placeIds={}",
                    crate::GAMES,
                    self
                ),
            )
            .await
            .expect("Failed to get game universe info");

        let fdata = client
            .request::<serde_json::Value>(
                Method::GET,
                &format!(
                    "{}/games?universeIds={}",
                    crate::GAMES,
                    udata[0]
                        .get("universeId")
                        .expect("Failed to find game universe ID")
                ),
            )
            .await
            .expect("Failed to get game root info");

        Ok(Game {
            client: client.clone(),
            ..serde_json::from_value(
                fdata.get("data").expect("Failed to get game root data")[0].clone(),
            )
            .expect("Failed to parse into Game")
        })
        // client.request::<Game>(Method::GET,
        //     &format!(
        //         "{}/games/multiget-place-details?placeIds={}",
        //         crate::GAMES,
        //         self
        //     )).await
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
                .request::<serde_json::Value>(
                    Method::GET,
                    &format!(
                        "{}/games/{}/servers/Public?limit=100",
                        crate::GAMES,
                        self.place_id
                    ),
                )
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
                    .request(
                        Method::GET,
                        &format!(
                            "{}/games/{}/servers/Public?limit=100&cursor={}",
                            crate::GAMES,
                            self.place_id,
                            cursor.as_str().unwrap()
                        ),
                    )
                    .await
                    .expect("Failed to get server list");

                if let Some(error) = data.get("errors") {
                    if let Some(message) = error[0].get("message") {
                        if message.as_str().unwrap() == "TooManyRequests" {
                            println!("Rate limited, sleeping for 3 seconds");
                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

                            data = self
                                .client
                                .request(
                                    Method::GET,
                                    &format!(
                                        "{}/games/{}/servers/Public?limit=100&cursor={}",
                                        crate::GAMES,
                                        self.place_id,
                                        cursor.as_str().unwrap()
                                    ),
                                )
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
    /// // Check if any errrs occurred
    /// if let Some(error) = data.get("errors") {
    ///     if error[0]["code"] == 4 {
    ///         // Developer Product Already exists
    ///         // Get the developer product internally here?
    ///         panic!("Product already exists");
    ///     } else {
    ///         panic!("Err: {}", error[0]);
    ///     }
    /// }
    ///
    pub async fn create_dev_product(&mut self, name: &str, price: u32) -> ApiResult<DevProduct> {
        // Make Request To DeveloperProducts
        self.client
            .request::<DevProduct>(
                Method::POST,
                &format!(
                    "{}/{}/developerproducts?name={}&description={}&priceInRobux={}",
                    crate::DEVPAGE,
                    self.universe_id,
                    name,
                    price,
                    price
                ),
            )
            .await
    }
}
