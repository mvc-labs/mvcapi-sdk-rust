/*
 * MicrovisionChain API Document
 *
 * API definition for MicrovisionChain provided apis
 *
 * The version of the OpenAPI document: 3.0.11
 * Contact: heqiming@metasv.com
 * Generated by: https://openapi-generator.tech
 */

/// XpubRequest : Request object to register(or delete) a new xpub



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct XpubRequest {
    /// The xpub to register.
    #[serde(rename = "xpub", skip_serializing_if = "Option::is_none")]
    pub xpub: Option<String>,
    /// Skip transactions before this height. Default is 0. Ignore this while deleting xpub.
    #[serde(rename = "skipHeight", skip_serializing_if = "Option::is_none")]
    pub skip_height: Option<i64>,
    /// Set the init maxReceiveIndex to {initReceiveIndex}(less than 5000) before the initial crawl , default is 200. This could increase cost.
    #[serde(rename = "initReceiveIndex", skip_serializing_if = "Option::is_none")]
    pub init_receive_index: Option<i32>,
    /// Set the init maxChangeIndex(less than 5000) before the initial crawl , default is 200. This could increase cost.
    #[serde(rename = "initChangeIndex", skip_serializing_if = "Option::is_none")]
    pub init_change_index: Option<i32>,
}

impl XpubRequest {
    /// Request object to register(or delete) a new xpub
    pub fn new() -> XpubRequest {
        XpubRequest {
            xpub: None,
            skip_height: None,
            init_receive_index: None,
            init_change_index: None,
        }
    }
}


