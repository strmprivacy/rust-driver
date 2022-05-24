![Crates.io](https://img.shields.io/crates/v/strm-privacy-driver)
# What is STRMPrivacy
[STRMPrivacy](https://strmprivacy.io/) is a privacy (and security) focused data processing platform. Define your data shape and the privacy implications in a data contract, and we take care of all the necessary transformations and split your data into privacy streams, a dedicated purpose-bound data interface that contains, for instance, only anonymized data.

## STRMPrivacy rust driver

this crate provides a convenient driver to authenticate and send events to the
STRMPrivacy api.

The driver consists out of 2 main components
- `StrmPrivacyClient`: Client to connect to the STRMPrivacy backend
- `StrmPrivacyValue`: Trait to convert structs and schemas into values that the client can send

### Generating structure based on a STRMPrivacy schema
When using our `cli` we can generate Rust code based on a `schema` we have created using the following command:
`strm get schema-code <*handle*/*schema_name*/*version*> --language=rust`

This will generate a rust project with the necessary scripts to generate a new package to be used in your project

### Example usage
Using and initializing the STRMPrivacy client to send event data to our api.
```rust
    use strm_privacy_driver::{StrmPrivacyClient, StrmPrivacyValue, StrmStatusCode};
    use std::env;
    use strm_privacy_driver::test::demo::{DemoEvent, StrmMeta};
    use strm_privacy_driver::error::Error;

    #[tokio::main]
    async fn main() -> Result<(), Error> {

        // initialize the env variables
        let billing_id = env::var("BILLING_ID").expect("no BILLING_ID found in environment");
        let client_id = env::var("CLIENT_ID").expect("no CLIENT_ID found in environment");
        let client_secret = env::var("CLIENT_SECRET").expect("no CLIENT_SECRET found in environment");

        let mut strm_privacy_client = StrmPrivacyClient::default(billing_id, client_id, client_secret).await?;

        let event = create_event();

        // catch specific status_codes and decide what to do
        match strm_privacy_client.send_event(event).await? {
            (StrmStatusCode::NO_CONTENT, _) => {}
            (status_code, message) => {assert!(false)}
        }

        Ok(())
    }

    // create new event based on the example schema
    fn create_event() -> DemoEvent {
        DemoEvent {
            strm_meta: StrmMeta {
                event_contract_ref: "strmprivacy/example/1.3.0".to_string(),
                nonce: None,
                timestamp: None,
                key_link: None,
                billing_id: None,
                consent_levels: vec![0],
            },
            unique_identifier: Some("unique".to_string()),
            consistent_value: "consistent".to_string(),
            some_sensitive_value: Some("sensitive".to_string()),
            not_sensitive_value: Some("not sensitive".to_string()),
        }
    }
```

# Contributing
If you encounter issues while using the chart, please check whether the issue you encounter has already been listed in the issue list. If not, feel free to create an issue.

Pull requests are greatly appreciated.
