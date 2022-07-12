use std::env;
use std::env::VarError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
}

impl Config {
    pub fn init() -> Result<Self, VarError> {
        let client_id = env::var("CLIENT_ID")?;
        let client_secret = env::var("CLIENT_SECRET")?;
        Ok(Self {
            client_secret,
            client_id,
        })
    }
}
