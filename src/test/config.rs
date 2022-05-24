use std::env;
use std::env::VarError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "billingId")]
    pub billing_id: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
}

impl Config {
    pub fn init() -> Result<Self, VarError> {
        let billing_id = env::var("BILLING_ID")?;
        let client_id = env::var("CLIENT_ID")?;
        let client_secret = env::var("CLIENT_SECRET")?;
        Ok(Self {
            billing_id,
            client_secret,
            client_id,
        })
    }
}
