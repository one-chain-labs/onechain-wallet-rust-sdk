use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferOrderQueryReq {
    /// Transaction hash
    pub hash: String,
    /// Recipient address
    pub to_address: String,
    /// Transfer currency
    pub currency: String,
    /// Status list
    pub status_list: Option<Vec<String>>,
    /// Start time yyyy-MM-dd
    pub begin_time: Option<i64>,
    /// End time yyyy-MM-dd
    pub end_time: Option<i64>,
    /// Completion start time yyyy-MM-dd
    pub complete_begin_time: Option<i64>,
    /// Completion end time yyyy-MM-dd
    pub complete_end_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferOrderQueryPageReq {
    /// User DID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    /// User address
    pub address: String,
    /// Order ID
    pub order_id: Option<String>,
    /// Transaction hash
    pub trade_hash: Option<String>,
    /// Recipient user DID
    pub to_did: Option<String>,
    /// Recipient address
    pub to_address: Option<String>,
    /// Minimum amount
    pub min_amount: Option<String>,
    /// Maximum amount
    pub max_amount: Option<String>,
    /// Transfer currency
    pub currency: Option<String>,
    /// Transfer method: DID,ADDRESS,NAME
    pub transfer_method: Option<String>,
    /// Status list
    pub status_list: Option<Vec<String>>,
    /// Start time yyyy-MM-dd
    pub begin_time: Option<i64>,
    /// End time yyyy-MM-dd
    pub end_time: Option<i64>,
    /// Query direction 0: in 1: out 2: in or out 3: both in and out
    pub query_type: Option<String>,
    /// Completion start time yyyy-MM-dd
    pub complete_begin_time: Option<i64>,
    /// Completion end time yyyy-MM-dd
    pub complete_end_time: Option<i64>,
    pub page_index: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferOrderReq {
    /// Sender address
    pub from_address: String,
    /// Recipient address
    pub to_address: String,
    /// Transfer coinType
    pub coin_type: String,
    /// Transfer amount
    pub amount: String,
    /// Remark
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferOrderTxReq {
    /// Transaction hash
    pub hash: String,
    /// Raw transaction content
    pub tx_bytes: String,
    /// User signature
    pub user_sig: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferOrderResp {
    /// Transaction hash
    pub hash: String,
    /// Sender user DID
    pub did: Option<String>,
    /// Sender user nickname
    pub nick_name: Option<String>,
    /// Sender account
    pub address: String,
    /// Sender HCname
    pub address_name: Option<String>,
    /// Sender application
    pub merchant_id: String,
    pub merchant_name: String,
    /// Transfer type
    pub transfer_method: String,
    /// Recipient user DID
    pub to_did: Option<String>,
    /// Recipient user nickname
    pub to_nick_name: Option<String>,
    /// Recipient account
    pub to_address: String,
    /// Recipient HCname
    pub to_address_name: Option<String>,
    /// Merchant ID of the recipient address
    pub to_merchant_id: String,
    /// Merchant name of the recipient address
    pub to_merchant_name: String,
    /// Currency
    pub currency: String,
    /// Amount
    pub amount: String,
    /// Status
    pub status: String,
    /// Initiation time
    pub create_time: i64,
    /// Completion time
    pub complete_time: i64,
    /// Remark
    pub remark: String,
    /// Sender name -> displayed according to transfer method
    pub sender: String,
    /// Recipient name -> displayed according to transfer method
    pub receiver: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferOrderTxResponse {
    /// Order status UN_PAY:pending payment, RUNNING:transferring, SUCCESS:transfer successful, FAIL:transfer failed, CANCEL:cancelled, TIMEOUT:timeout
    pub status: String,
    /// Transaction hash
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildSponsorTxReq {
    /// Sender address
    pub address: String,
    /// Unsigned transaction (base64)
    pub raw_transaction: String,
    /// Whether to build offline
    pub only_transaction_kind: bool,
    /// Gas limit (e.g: 0.001), default 9
    pub gas_budget: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasTxBuilderResponse {
    // Hash
    pub hash: String,
    /// Transaction to be signed
    pub raw_transaction: String,
    /// Transaction expiration time
    pub expiration: i64,
    /// Fee sponsorship address
    pub sponsor: String,
    /// Reservation ID
    pub reservation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyPayTxReq {
    /// User signature
    pub user_sig: String,
    /// Raw data
    pub tx_bytes: String,
    /// Reservation ID
    pub reservation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyPayTxResp {
    pub hash: String,
    pub status: bool,
}
