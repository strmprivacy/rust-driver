use crate::strm_privacy_value::StrmPrivacyValue;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct StrmMeta {
    #[serde(rename = "eventContractRef")]
    pub event_contract_ref: String,
    pub nonce: Option<i32>,
    pub timestamp: Option<i64>,
    #[serde(rename = "keyLink")]
    pub key_link: Option<String>,
    #[serde(rename = "billingId")]
    pub billing_id: Option<String>,
    #[serde(rename = "consentLevels")]
    pub consent_levels: Vec<i32>,
}

impl Default for StrmMeta {
    fn default() -> StrmMeta {
        StrmMeta {
            event_contract_ref: String::default(),
            nonce: None,
            timestamp: None,
            key_link: None,
            billing_id: None,
            consent_levels: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DemoEvent {
    #[serde(rename = "strmMeta")]
    pub strm_meta: StrmMeta,
    #[serde(rename = "uniqueIdentifier")]
    pub unique_identifier: Option<String>,
    #[serde(rename = "consistentValue")]
    pub consistent_value: String,
    #[serde(rename = "someSensitiveValue")]
    pub some_sensitive_value: Option<String>,
    #[serde(rename = "notSensitiveValue")]
    pub not_sensitive_value: Option<String>,
}

impl Default for DemoEvent {
    fn default() -> DemoEvent {
        DemoEvent {
            strm_meta: StrmMeta::default(),
            unique_identifier: None,
            consistent_value: String::default(),
            some_sensitive_value: None,
            not_sensitive_value: None,
        }
    }
}

impl StrmPrivacyValue for DemoEvent {
    const STRM_SCHEMA_REF: &'static str = "strmprivacy/demo/1.0.2";
    const STRM_SCHEMA: &'static str = r#"{
    "type": "record",
    "name": "DemoEvent",
    "namespace": "io.strmprivacy.schemas.demo.v1",
    "fields": [
        {
            "name": "strmMeta",
            "type": {
                "type": "record",
                "name": "StrmMeta",
                "fields": [
                    {
                        "name": "eventContractRef",
                        "type": "string"
                    },
                    {
                        "name": "nonce",
                        "type": [
                            "null",
                            "int"
                        ],
                        "default": null
                    },
                    {
                        "name": "timestamp",
                        "type": [
                            "null",
                            "long"
                        ],
                        "default": null
                    },
                    {
                        "name": "keyLink",
                        "type": [
                            "null",
                            "string"
                        ],
                        "default": null
                    },
                    {
                        "name": "billingId",
                        "type": [
                            "null",
                            "string"
                        ],
                        "default": null
                    },
                    {
                        "name": "consentLevels",
                        "type": {
                            "type": "array",
                            "items": "int"
                        }
                    }
                ]
            }
        },
        {
            "name": "uniqueIdentifier",
            "type": [
                "null",
                "string"
            ],
            "default": null,
            "doc": "any value. For illustration purposes: use a value that is consistent over time like a customer or device ID."
        },
        {
            "name": "consistentValue",
            "type": "string",
            "doc": "any value. For illustration purposes: use a value that is consistent over a limited period like a session."
        },
        {
            "name": "someSensitiveValue",
            "type": [
                "null",
                "string"
            ],
            "default": null,
            "doc": "any value. For illustration purposes: use a value that could identify a user over time based on behavior, like browsing behavior (e.g. urls)."
        },
        {
            "name": "notSensitiveValue",
            "type": [
                "null",
                "string"
            ],
            "default": null,
            "doc": "any value. For illustration purposes: use a value that is not sensitive at all, like the rank of an item in a set."
        }
    ]
}"#;
}
