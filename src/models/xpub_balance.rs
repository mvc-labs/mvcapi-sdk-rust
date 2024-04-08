/*
 * MicrovisionChain API Document
 *
 * API definition for MicrovisionChain provided apis
 *
 * The version of the OpenAPI document: 3.0.11
 * Contact: heqiming@metasv.com
 * Generated by: https://openapi-generator.tech
 */

/// XpubBalance : Registered Xpub balance including confirmed and unconfirmed



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct XpubBalance {
    /// confirmed balance
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<i64>,
    /// unconfirmed balance
    #[serde(rename = "unconfirmed", skip_serializing_if = "Option::is_none")]
    pub unconfirmed: Option<i64>,
}

impl XpubBalance {
    /// Registered Xpub balance including confirmed and unconfirmed
    pub fn new() -> XpubBalance {
        XpubBalance {
            confirmed: None,
            unconfirmed: None,
        }
    }
}


