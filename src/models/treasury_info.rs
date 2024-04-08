/*
 * MicrovisionChain API Document
 *
 * API definition for MicrovisionChain provided apis
 *
 * The version of the OpenAPI document: 3.0.11
 * Contact: heqiming@metasv.com
 * Generated by: https://openapi-generator.tech
 */

/// TreasuryInfo : current treasury utxo info



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TreasuryInfo {
    /// current treasury utxo txid
    #[serde(rename = "txid", skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    /// current treasury utxo index
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// current treasury amount
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// current treasury utxo height
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// current treasury utxo block hash
    #[serde(rename = "blockHash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// current treasury utxo timestamp
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

impl TreasuryInfo {
    /// current treasury utxo info
    pub fn new() -> TreasuryInfo {
        TreasuryInfo {
            txid: None,
            index: None,
            amount: None,
            height: None,
            block_hash: None,
            timestamp: None,
        }
    }
}


