use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::time::SystemTime;

pub const ACCESS_TOKEN: &'static str = "ACCESS_TOKEN";
pub const TOKEN_ID: &'static str = "TOKEN_ID";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseReq<T: Serialize + Debug> {
    /// Timestamp, in milliseconds
    pub timestamp: u128,
    /// Merchant signature
    pub merchant_sign: String,
    /// Merchant ID
    pub merchant_id: String,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub body: Option<T>,
}

impl<T: Serialize + Debug> BaseReq<T> {
    pub fn new(merchant_id: String, body: Option<T>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self {
            timestamp,
            merchant_id,
            merchant_sign: "".to_string(),
            body,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonResp<T = String> {
    /// Status code
    pub code: String,
    /// Response message
    pub msg: String,
    /// Response data
    pub data: Option<T>,
    /// Whether successful
    pub success: bool,
    pub trace_id: String,
    pub system_time: i64,
}

impl<T> CommonResp<T> {
    pub const SUCCESS_RESP_CODE: &'static str = "000000";
    pub const SUCCESS_MSG: &'static str = "success";
}

impl<T: Clone> CommonResp<T> {
    pub fn get_data(&self) -> anyhow::Result<T> {
        if self.success {
            Ok(self.data.clone().unwrap())
        } else {
            anyhow::bail!(
                "code: {} msg:{} trace:{}",
                self.code.clone(),
                self.msg.clone(),
                self.trace_id.clone()
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResp {
    /// Transaction hash
    pub hash: String,
    /// Raw transaction data to be signed
    pub raw_transaction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T = String> {
    /// Paged data
    pub rows: Vec<T>,
    /// Total number
    pub total_num: i64,
    /// Page size
    pub page_size: i64,
    /// Current page index
    pub page_index: i64,
}
