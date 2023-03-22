use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(skip)]
    pub client: Option<crate::Https>,
    #[serde(skip)]
    pub friends: Option<Vec<User>>,

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
