/*
 * MicrovisionChain API Document
 *
 * API definition for MicrovisionChain provided apis
 *
 * The version of the OpenAPI document: 3.0.11
 * Contact: heqiming@metasv.com
 * Generated by: https://openapi-generator.tech
 */

/// TxInput : Parsed inputs from raw tx. Use output api to get value and spent info.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TxInput {
    /// Input index of the tx.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// The outpoint utxo txid that this input spent
    #[serde(rename = "utxoTxid", skip_serializing_if = "Option::is_none")]
    pub utxo_txid: Option<String>,
    /// The outpoint utxo index that this input spent
    #[serde(rename = "utxoIndex", skip_serializing_if = "Option::is_none")]
    pub utxo_index: Option<i32>,
    /// Parsed address from pubkey(P2PKH input only).
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// satoshi value of this input.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// The hex of the input script.
    #[serde(rename = "unlockScript", skip_serializing_if = "Option::is_none")]
    pub unlock_script: Option<String>,
}

impl TxInput {
    /// Parsed inputs from raw tx. Use output api to get value and spent info.
    pub fn new() -> TxInput {
        TxInput {
            index: None,
            utxo_txid: None,
            utxo_index: None,
            address: None,
            value: None,
            unlock_script: None,
        }
    }
}

