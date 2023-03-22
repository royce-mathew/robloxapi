use reqwest::{header, Response, RequestBuilder, Method};

#[derive(Debug, Clone)]
pub struct Https {
    pub client: reqwest::Client,
    xcsrftoken: Option<String>
}

impl Default for Https {
    fn default() -> Self {
        Self::new()
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

            xcsrftoken: None,
        }
    }

    // Send a get_request. Automatically handles the x-csrf token regeneration
    pub async fn request(&mut self, method: Method, request_url: &str) -> Result<serde_json::Value, reqwest::Error> {
        let req_build = self
            .client
            .request(method.clone(), request_url);

        // Keep building the request
        // Check if token exists
        let init_request = if let Some(token) = &self.xcsrftoken {
            req_build.header(
                "x-csrf-token",
                header::HeaderValue::from_str(token).unwrap(),
            )
        } else {req_build}
            .send()
            .await?;

        // Extract headers from request
        let headers = init_request.headers().clone();
        
        // Check if any errrs occurred
        if init_request.status().is_client_error() {
            self.xcsrftoken = headers.get("x-csrf-token").map(|value| value.to_str().unwrap().to_owned());
            let second_request = self.client
                .request(method, request_url)
                .header(
                    "x-csrf-token",
                    headers.get("x-csrf-token").unwrap(),
                )
                .send()
                .await;
            return Ok(serde_json::from_str::<serde_json::Value>(&second_request.unwrap().text().await?).unwrap());
        }

        let json_data = init_request.json::<serde_json::Value>().await?;
        Ok(json_data)
    }

    pub async fn post_default(&mut self, request_url: &str) -> Result<Response, reqwest::Error> {
        // if self.xcsrftoken == None {
        //     self.xcsrftoken = self.create_xcsrf_token().await;
        // }

        let request = self
            .client
            .post(request_url)
            .header(
                "x-csrf-token",
                header::HeaderValue::from_str(self.xcsrftoken.as_ref().unwrap()).unwrap(),
            )
            .header("content-length", "0")
            .send()
            .await;

        self.xcsrftoken = request.as_ref().unwrap().headers()
            .get("x-csrf-token")
            .map(|value| value.to_str().unwrap().to_owned());

        request
    }

    pub async fn post(&mut self, request_url: &str) -> RequestBuilder {
        self
            .client
            .post(request_url)
            .header(
                "x-csrf-token",
                header::HeaderValue::from_str(self.xcsrftoken.clone().unwrap().as_str()).unwrap(),
            )
    }

    pub async fn set_cookie(mut self, cookie: &str) -> Self {
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
        // headers.insert(
        //     header::HeaderName::from_static("x-csrf-token"),
        //     header::HeaderValue::from_str( self.xcsrftoken.clone().unwrap().as_str()).unwrap(),
        // );

        // Create a new session with the cookie and token
        self.client = reqwest::Client::builder()
            .cookie_store(true)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.152 Safari/537.36")
            .default_headers(headers)
            .build()
            .expect("Failed to build new client from headers");

        // self.validate_cookie().await;
        self
    }

    // // Validate the cookie
    // async fn validate_cookie(&mut self) {
    //     let request = self
    //         .request(Method::GET, "https://www.roblox.com/mobileapi/userinfo")
    //         .await
    //         .expect("Failed to get user info");

    //     self.xcsrftoken =  request.headers()
    //         .get("x-csrf-token")
    //         .map(|value| value.to_str().unwrap().to_owned());

    //     let _: serde_json::Value = request.json().await.expect("Failed to get json");
    // }
}


