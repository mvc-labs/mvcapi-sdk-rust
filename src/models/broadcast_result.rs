/*
 * MicrovisionChain API Document
 *
 * API definition for MicrovisionChain provided apis
 *
 * The version of the OpenAPI document: 3.0.11
 * Contact: heqiming@metasv.com
 * Generated by: https://openapi-generator.tech
 */

/// BroadcastResult : Broadcast result



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BroadcastResult {
    /// return txid if broadcast success
    #[serde(rename = "txid", skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    /// return messages if broadcast failed
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl BroadcastResult {
    /// Broadcast result
    pub fn new() -> BroadcastResult {
        BroadcastResult {
            txid: None,
            message: None,
        }
    }
}


