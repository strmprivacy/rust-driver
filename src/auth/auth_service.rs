use crate::auth::tokens::{AccessToken, Token};
use reqwest::{Client, Error};

#[derive(Serialize, Debug, Default)]
pub struct AuthService {
    #[serde(rename = "clientId")]
    client_id: String,
    #[serde(rename = "clientSecret")]
    client_secret: String,
    #[serde(skip_serializing)]
    pub token: Token,
    #[serde(skip_serializing)]
    auth_url: &'static str,
}

impl AuthService {
    pub fn new(
        client_id: String,
        client_secret: String,
        auth_url: &'static str,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            auth_url,
            ..Default::default()
        }
    }

    // authenticate the STRMPrivacy client
    pub async fn authenticate(&mut self, client: &Client) -> Result<(), Error> {
        let token = client
            .post(self.auth_url)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(self.format_body_authenticate())
            .send()
            .await?
            .json::<Token>()
            .await?;

        self.token = token;
        Ok(())
    }

    // refresh the accessToken
    pub async fn refresh(&mut self, client: &Client) -> Result<(), Error> {
        let token = client
            .post(self.auth_url)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(self.format_body_refresh(&self.token))
            .send()
            .await?
            .json::<AccessToken>()
            .await?;

        self.token.access_token = token.access_token;
        Ok(())
    }

    pub fn format_body_authenticate(&self) -> String {
        format!("grant_type=client_credentials&client_id={}&client_secret={}", self.client_id, self.client_secret)
    }

    pub fn format_body_refresh(&self, token: &Token) -> String {
        format!("grant_type=refresh_token&client_id={}&client_secret={}&refresh_token={}", self.client_id, self.client_secret, token.refresh_token)
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::auth_service::AuthService;
    use crate::strm_privacy_client::AUTH_URL;
    use crate::test::config::Config;
    use dotenv::dotenv;
    use reqwest::Client;

    #[tokio::test]
    async fn test_authenticate() {
        dotenv().ok();
        let config_result = Config::init();
        assert!(config_result.is_ok());
        let client = Client::new();
        let config = config_result.unwrap();
        let mut auth_service = AuthService::new(
            config.client_id,
            config.client_secret,
            AUTH_URL
        );

        assert!(auth_service.authenticate(&client).await.is_ok());
    }

    #[tokio::test]
    async fn test_refresh() {
        dotenv().ok();
        let client = Client::new();
        let config_result = Config::init();
        assert!(config_result.is_ok());
        let config = config_result.unwrap();
        let mut auth_service = AuthService::new(
            config.client_id,
            config.client_secret,
            AUTH_URL
        );

        assert!(auth_service.authenticate(&client).await.is_ok());
        assert!(auth_service.refresh(&client).await.is_ok());
    }
}
