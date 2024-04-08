/*
 * MicrovisionChain API Document
 *
 * API definition for MicrovisionChain provided apis
 *
 * The version of the OpenAPI document: 3.0.11
 * Contact: heqiming@metasv.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BlockchainInfo {
    /// mainnet or testnet
    #[serde(rename = "chain", skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// current block count
    #[serde(rename = "blocks", skip_serializing_if = "Option::is_none")]
    pub blocks: Option<i32>,
    /// current block header count
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<i32>,
    /// current highest blockhash
    #[serde(rename = "bestBlockHash", skip_serializing_if = "Option::is_none")]
    pub best_block_hash: Option<String>,
    /// decimal string for current difficulty
    #[serde(rename = "difficulty", skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<String>,
    /// current median time
    #[serde(rename = "medianTime", skip_serializing_if = "Option::is_none")]
    pub median_time: Option<i64>,
    /// current pow
    #[serde(rename = "chainwork", skip_serializing_if = "Option::is_none")]
    pub chainwork: Option<String>,
    /// estimated current network hash rate.
    #[serde(rename = "networkHashPerSecond", skip_serializing_if = "Option::is_none")]
    pub network_hash_per_second: Option<String>,
    /// current mempool transaction count.
    #[serde(rename = "mempoolTxCount", skip_serializing_if = "Option::is_none")]
    pub mempool_tx_count: Option<i32>,
    /// current mempool usage.
    #[serde(rename = "mempoolUsage", skip_serializing_if = "Option::is_none")]
    pub mempool_usage: Option<i64>,
    /// next estimated block size.
    #[serde(rename = "estimatedBlockSize", skip_serializing_if = "Option::is_none")]
    pub estimated_block_size: Option<i64>,
}

impl BlockchainInfo {
    pub fn new() -> BlockchainInfo {
        BlockchainInfo {
            chain: None,
            blocks: None,
            headers: None,
            best_block_hash: None,
            difficulty: None,
            median_time: None,
            chainwork: None,
            network_hash_per_second: None,
            mempool_tx_count: None,
            mempool_usage: None,
            estimated_block_size: None,
        }
    }
}

