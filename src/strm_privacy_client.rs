use reqwest::{Client, Error, StatusCode};

use crate::auth::auth_service::AuthService;
use crate::client::sender_service::SenderService;
use crate::strm_privacy_value::StrmPrivacyValue;

pub const AUTH_URL: &str = "https://accounts.strmprivacy.io/auth/realms/streams/protocol/openid-connect/token";
pub const API_URL: &str = "https://events.strmprivacy.io/event";

/// Container type to easily return backend responses from `send_event` function
pub type StrmPrivacyResponse = (StrmStatusCode, String);

const MAX_RETRIES: usize = 3;

/// type alias for match status codes after `send_events` function
pub type StrmStatusCode = StatusCode;

pub struct StrmPrivacyClient {
    sender_service: SenderService,
    auth_service: AuthService,
    client: Client,
}

impl StrmPrivacyClient {
    /// Creates a new `StrmPrivacyClient` with custom endpoints
    pub async fn new(
        client_id: String,
        client_secret: String,
        auth_url: &'static str,
        api_url: &'static str,
    ) -> Result<Self, Error> {
        let client = Client::new();
        let mut auth_service = AuthService::new(client_id, client_secret, auth_url);
        let sender_service = SenderService::new(api_url);
        auth_service.authenticate(&client).await?;
        Ok(Self {
            sender_service,
            auth_service,
            client,
        })
    }

    /// Creates a new `StrmPrivacyClient` with default endpoints
    pub async fn default(
        client_id: String,
        client_secret: String,
    ) -> Result<Self, Error> {
        let client = Client::new();
        let mut auth_service = AuthService::new(client_id, client_secret, AUTH_URL);
        let sender_service = SenderService::new(API_URL);
        auth_service.authenticate(&client).await?;
        Ok(Self {
            sender_service,
            auth_service,
            client,
        })
    }

    /// Send a `StrmPrivacyValue` to the backend en return the corresponding response
    pub async fn send_event<T>(&mut self, event: T) -> Result<StrmPrivacyResponse, Error>
    where
        T: StrmPrivacyValue,
    {
        let mut response: StrmPrivacyResponse = (StrmStatusCode::NO_CONTENT, "".to_string());
        for i in 0..MAX_RETRIES {
            let resp = self
                .sender_service
                .send_event(&self.client, &self.auth_service.token, event.clone())
                .await?;

            let status_code = resp.status();
            let message = resp.text().await?;
            response = (status_code, message);

            // refresh the token otherwise break out of the loop
            match status_code {
                // for the last refresh possibility it doesn't make sense
                // to try to refresh again as we can't catch it to redo the
                // request
                StrmStatusCode::UNAUTHORIZED if i + 1 < MAX_RETRIES => {
                    self.refresh().await?;
                }
                _ => {
                    break;
                }
            }
        }

        Ok(response)
    }

    async fn refresh(&mut self) -> Result<(), Error> {
        self.auth_service.refresh(&self.client).await
    }
}
