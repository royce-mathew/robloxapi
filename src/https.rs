use crate::{errors::ApiError, errors::RobloxApiErrorResponse, ApiResult, Client};
use reqwest::{header, Method, RequestBuilder, Response};
use serde::de::{self, DeserializeOwned};

#[derive(Debug, Clone)]
pub struct Https {
    pub client: reqwest::Client,
}

impl Default for Https {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// # setCookie
    /// Set the cookie for the client; This function is needed to execute specific API requests such as `.create_developer_product()`
    ///
    /// # Example
    /// ```
    ///
    /// let COOKIE: &str = "_|WARNING:-DO-NOT-SHARE-THIS.--Sharing-this-will-allow-someone-to-log-in-as-you-and-to-steal-your-ROBUX-and-items.|_8B1028";
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = robloxapi::Client();
    ///     client.set_cookie(COOKIE).await;
    /// }
    ///
    /// ```
    pub async fn set_cookie(&mut self, cookie: &str) -> &mut Self {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&(".ROBLOSECURITY=".to_owned() + cookie)).unwrap(),
        );
        headers.insert(
            header::CONTENT_LENGTH,
            header::HeaderValue::from_static("0"),
        );

        // Add the x-csrf-token to the headers
        headers.insert(
            header::HeaderName::from_static("x-csrf-token"),
            header::HeaderValue::from(
                reqwest::Client::new()
                    .post("https://auth.roblox.com/v2/logout")
                    .header("content-length", "0")
                    .send()
                    .await
                    .expect("Failed to get X-CSRF-TOKEN")
                    .headers()
                    .get("x-csrf-token")
                    .unwrap_or(&header::HeaderValue::from_static("")),
            ),
        );

        // Create a new session with the cookie and token
        self.session.client = reqwest::Client::builder()
            .cookie_store(true)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.152 Safari/537.36")
            .default_headers(headers)
            .build()
            .expect("Failed to build new client from headers");

        // Validate Cookie before continuing
        self.session.validate_cookie().await;

        self
    }
}

impl Https {
    /// Create a new client instance
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
        }
    }

    async fn de_to_result<T>(req: Response) -> ApiResult<T>
    where
        T: DeserializeOwned,
    {
        let status_code = req.status();
        let data = req.bytes().await?;

        if let Ok(error) = serde_json::from_slice::<RobloxApiErrorResponse>(&data) {
            if !error.is_empty() {
                return Err(ApiError::Roblox {
                    status_code,
                    reason: error.reason().unwrap_or_else(|| "Unknown error".to_owned()),
                });
            }
        }
        Ok(serde_json::from_slice::<T>(&data)?)
    }

    // Send a get_request. Automatically handles the x-csrf token regeneration
    pub async fn request<T>(&mut self, method: Method, request_url: &str) -> ApiResult<T>
    where
        T: de::DeserializeOwned,
    {
        println!("{}", request_url);
        let response = self
            .client
            .request(method.clone(), request_url)
            .send()
            .await
            .expect("Request failed");

        return Https::de_to_result::<T>(response).await;
    }

    pub async fn post(&mut self, request_url: &str) -> RequestBuilder {
        self.client.post(request_url)
    }

    // Validate the cookie
    async fn validate_cookie(&mut self) {
        let req = self
            .client
            .request(Method::GET, "https://www.roblox.com/mobileapi/userinfo")
            .send()
            .await
            .expect("Failed to get user info");

        let _: serde_json::Value = req.json().await.expect("Failed to validate cookie");
    }
}
