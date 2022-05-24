// `expiredAt` is also present in the response but we are not using it
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Token {
    #[serde(skip_serializing)]
    #[serde(rename = "idToken")]
    id_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
}

impl Token {
    pub fn format_bearer(&self) -> String {
        format!("Bearer {}", self.id_token)
    }
}
