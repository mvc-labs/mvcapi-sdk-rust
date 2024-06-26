/*
 * MicrovisionChain API Document
 *
 * API definition for MicrovisionChain provided apis
 *
 * The version of the OpenAPI document: 3.0.11
 * Contact: heqiming@metasv.com
 * Generated by: https://openapi-generator.tech
 */

/// UtxoPickRequest : Request object to batch pick utxo



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UtxoPickRequest {
    /// The address list to pick utxo from
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    /// The total amount you want to pick
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}

impl UtxoPickRequest {
    /// Request object to batch pick utxo
    pub fn new() -> UtxoPickRequest {
        UtxoPickRequest {
            addresses: None,
            amount: None,
        }
    }
}


