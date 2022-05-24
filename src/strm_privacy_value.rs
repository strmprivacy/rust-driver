use avro_rs::types::Value;
use avro_rs::Schema;
use serde::Serialize;

pub trait StrmPrivacyValue: Serialize + Clone {
    const STRM_SCHEMA_REF: &'static str;
    const STRM_SCHEMA: &'static str;

    fn get_schema(schema: &'static str) -> Schema {
        Schema::parse_str(schema).unwrap()
    }

    fn avro_value(&self) -> Value {
        avro_rs::to_value(&self).unwrap()
    }
}
