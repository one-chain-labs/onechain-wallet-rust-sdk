use serde::{Deserialize, Serialize};

/// Chain currency information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyChainResp {
    /// Chain
    pub chain: String,

    /// Currency list
    pub currency_list: Vec<CurrencyInfo>,
}

/// Currency information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyInfo {
    /// Currency type 1:Fiat currency 2:Digital currency
    pub currency_type: i32,

    /// Currency code
    pub currency: String,

    /// Currency name
    pub name: String,

    /// Currency icon
    pub pic: String,

    /// USD exchange rate
    pub exchange_rate: String,

    /// Display precision
    pub display_decimals: i32,

    /// Calculation precision
    pub calculate_decimals: i32,

    /// Creation time
    pub create_time: i64,

    /// Update time
    pub update_time: i64,

    /// Currency symbol
    pub symbol: String,

    /// Currency collection address
    pub coin_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryWalletReq {
    /// User did, pass either this or wallet address
    pub did: Option<String>,
    /// User wallet address, pass either this or did
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWalletResp {
    ///  DID
    pub did: Option<String>,
    /// Login user number
    pub user_no: String,
    /// Address
    pub address: String,
    /// Chain
    pub chain: String,
    /// Account number
    pub account: String,
    /// Account name
    pub account_name: String,
    /// Wallet type
    pub wallet_type: String,
    /// Alias
    pub alias_name: Option<String>,
}
