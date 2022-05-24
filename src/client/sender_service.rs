use crate::auth::tokens::Token;
use crate::strm_privacy_value::StrmPrivacyValue;
use avro_rs::to_avro_datum;
use reqwest::{Client, Error, Response};

pub struct SenderService {
    api_url: &'static str,
}

impl SenderService {
    pub fn new(api_url: &'static str) -> Self {
        Self { api_url }
    }

    pub async fn send_event<T>(
        &self,
        client: &Client,
        token: &Token,
        event: T,
    ) -> Result<Response, Error>
    where
        T: StrmPrivacyValue,
    {
        client
            .post(self.api_url)
            .header("authorization", token.format_bearer())
            .header("Strm-Schema-Ref", T::STRM_SCHEMA_REF)
            .body(to_avro_datum(&T::get_schema(T::STRM_SCHEMA), event.avro_value()).unwrap())
            .send()
            .await
    }
}
