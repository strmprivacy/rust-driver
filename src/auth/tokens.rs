// `expiredAt` is also present in the response but we are not using it
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
}

/// used for retrieving `access_token` after refresh
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AccessToken {
    pub access_token: String
}

impl Token {
    pub fn format_bearer(&self) -> String {
        format!("Bearer {}", self.access_token)
    }
}
