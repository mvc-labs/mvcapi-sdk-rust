/*
 * MicrovisionChain API Document
 *
 * API definition for MicrovisionChain provided apis
 *
 * The version of the OpenAPI document: 3.0.11
 * Contact: heqiming@metasv.com
 * Generated by: https://openapi-generator.tech
 */

/// AddressBalance : The current balance for a particular address



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressBalance {
    /// current address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// confirmed balance
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<i64>,
    /// unconfirmed balance
    #[serde(rename = "unconfirmed", skip_serializing_if = "Option::is_none")]
    pub unconfirmed: Option<i64>,
}

impl AddressBalance {
    /// The current balance for a particular address
    pub fn new() -> AddressBalance {
        AddressBalance {
            address: None,
            confirmed: None,
            unconfirmed: None,
        }
    }
}


