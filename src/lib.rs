//! # STRMPrivacy rust driver
//!
//! this crate provides a convenient driver to authenticate and send events to the
//! STRMPrivacy api.
//!
//! The driver consists out of 2 main components
//! - `StrmPrivacyClient`: Client to connect to the STRMPrivacy backend
//! - `StrmPrivacyValue`: Trait to convert structs and schemas into values that the client can send
//!
//! ## Generating structure based on a STRMPrivacy schema
//! When using our `cli` we can generate Rust code based on a `schema` we have created using the following command:
//! `strm get schema-code <*handle*/*schema_name*/*version*> --language=rust`
//!
//! This will generate a rust project with the necessary scripts to generate a new package to be used in your project
//!
//! ## Example usage
//! Using and initializing the STRMPrivacy client to send event data to our api.
//! ```
//!     use strm_privacy_driver::{StrmPrivacyClient, StrmStatusCode};
//! #   use dotenv::dotenv;
//!     use std::env;
//!     use strm_privacy_driver::test::demo::{DemoEvent, StrmMeta};
//!     use strm_privacy_driver::error::Error;
//!
//!     #[tokio::main]
//!     async fn main() -> Result<(), Error> {
//! #       dotenv().ok();
//!
//!         // initialize the env variables
//!         let client_id = env::var("CLIENT_ID").expect("no CLIENT_ID found in environment");
//!         let client_secret = env::var("CLIENT_SECRET").expect("no CLIENT_SECRET found in environment");
//!
//!         let mut strm_privacy_client = StrmPrivacyClient::default(client_id, client_secret).await?;
//!
//!         let event = create_event();
//!
//!         // catch specific status_codes and decide what to do
//!         match strm_privacy_client.send_event(event).await? {
//!             (StrmStatusCode::NO_CONTENT, _) => {}
//!             (status_code, message) => {assert!(false)}
//!         }
//!
//!         Ok(())
//!     }
//!
//!     // create new event based on the example schema
//!     fn create_event() -> DemoEvent {
//!         DemoEvent {
//!             strm_meta: StrmMeta {
//!                 event_contract_ref: "strmprivacy/example/1.3.0".to_string(),
//!                 nonce: None,
//!                 timestamp: None,
//!                 key_link: None,
//!                 billing_id: None,
//!                 consent_levels: vec![0],
//!             },
//!             unique_identifier: Some("unique".to_string()),
//!             consistent_value: "consistent".to_string(),
//!             some_sensitive_value: Some("sensitive".to_string()),
//!             not_sensitive_value: Some("not sensitive".to_string()),
//!         }
//!     }
//! ```

#[macro_use]
extern crate serde;

mod auth;
mod client;
pub mod error;
pub(crate) mod strm_privacy_client;
pub(crate) mod strm_privacy_value;
pub mod test;

pub use strm_privacy_client::StrmPrivacyClient;
pub use strm_privacy_client::StrmPrivacyResponse;
pub use strm_privacy_client::StrmStatusCode;
pub use strm_privacy_value::StrmPrivacyValue;
