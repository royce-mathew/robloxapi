use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    #[serde(skip)]
    pub client: crate::Https,
    #[serde(skip)]
    pub servers: Option<Vec<Server>>,

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub id: String,
    #[serde(rename = "maxPlayers")]
    pub max_players: u8,
    pub playing: u32,
    pub fps: f32,
    pub ping: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DevProduct {
    #[serde(skip)]
    pub name: String,
    #[serde(skip)]
    pub price: u32,
    // #[serde(skip)]
    pub id: u64,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "iconImageAssetId")]
    pub image_asset_id: Option<u64>,
    #[serde(rename = "shopId")]
    pub shop_id: u64,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Game(placeid={}, name={})", self.place_id, self.name)
    }
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

impl std::fmt::Display for DevProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "DevProduct(name={}, price={}, product_id={})",
            self.name, self.price, self.id
        )
    }
}
