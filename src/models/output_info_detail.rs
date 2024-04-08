/*
 * MicrovisionChain API Document
 *
 * API definition for MicrovisionChain provided apis
 *
 * The version of the OpenAPI document: 3.0.11
 * Contact: heqiming@metasv.com
 * Generated by: https://openapi-generator.tech
 */

/// OutputInfoDetail : spent status and value info of the output with detailed output script.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OutputInfoDetail {
    /// txid that this output is in.
    #[serde(rename = "txid", skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    /// index of this output in the tx.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// output scrypt in hex format
    #[serde(rename = "script", skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    /// parsed address of this output, empty for non standard.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// value of this output
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// this output is spent or not, true if spent
    #[serde(rename = "spent", skip_serializing_if = "Option::is_none")]
    pub spent: Option<bool>,
    /// txid that spent this output
    #[serde(rename = "spentTxid", skip_serializing_if = "Option::is_none")]
    pub spent_txid: Option<String>,
    /// vin index of the spent tx
    #[serde(rename = "spentIndex", skip_serializing_if = "Option::is_none")]
    pub spent_index: Option<i32>,
    /// height of the spent tx(-1 if unconfirmed, 0 if unspent)
    #[serde(rename = "spentHeight", skip_serializing_if = "Option::is_none")]
    pub spent_height: Option<i64>,
}

impl OutputInfoDetail {
    /// spent status and value info of the output with detailed output script.
    pub fn new() -> OutputInfoDetail {
        OutputInfoDetail {
            txid: None,
            index: None,
            script: None,
            address: None,
            value: None,
            spent: None,
            spent_txid: None,
            spent_index: None,
            spent_height: None,
        }
    }
}


