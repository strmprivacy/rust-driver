use crate::auth::tokens::Token;
use reqwest::{Client, Error};

#[derive(Serialize, Debug, Default)]
pub struct AuthService {
    #[serde(rename = "billingId")]
    billing_id: String,
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
        billing_id: String,
        client_id: String,
        client_secret: String,
        auth_url: &'static str,
    ) -> Self {
        Self {
            billing_id,
            client_id,
            client_secret,
            auth_url,
            ..Default::default()
        }
    }

    // authenticate the STRMPrivacy client
    pub async fn authenticate(&mut self, client: &Client) -> Result<(), Error> {
        let token = client
            .post(self.auth_url())
            .json(&self)
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
            .post(self.refresh_url())
            .json(&self.token)
            .send()
            .await?
            .json::<Token>()
            .await?;

        self.token = token;

        Ok(())
    }

    fn auth_url(&self) -> String {
        format!("{}/auth", self.auth_url)
    }

    fn refresh_url(&self) -> String {
        format!("{}/refresh", self.auth_url)
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
            config.billing_id,
            config.client_id,
            config.client_secret,
            AUTH_URL,
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
            config.billing_id,
            config.client_id,
            config.client_secret,
            AUTH_URL,
        );

        assert!(auth_service.authenticate(&client).await.is_ok());
        assert!(auth_service.refresh(&client).await.is_ok());
    }
}
