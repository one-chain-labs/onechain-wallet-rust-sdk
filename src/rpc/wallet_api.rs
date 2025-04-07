use super::CallMethod;
use crate::types::common::CommonResp;
use crate::types::wallet::{QueryWalletReq, UserWalletResp};
use crate::{rpc::Call, types::wallet::CurrencyChainResp};
use async_trait::async_trait;

/// Wallet API interface trait for OneChain Wallet SDK
///
/// This trait provides methods for interacting with wallet-related functionalities,
/// including querying chain currency information and user wallet details.
#[async_trait]
pub trait WalletApi: Call {
    const BASE_PATH: &'static str = "/wallet";

    /// Retrieves a list of supported chain currencies
    ///
    /// This method queries the available blockchain currencies and their related information
    /// from the OneChain platform.
    ///
    /// - `Result<CommonResp<Vec<CurrencyChainResp>>>`: A list of currency chain information on success
    async fn query_chain_currency_for_list(
        &self,
    ) -> anyhow::Result<CommonResp<Vec<CurrencyChainResp>>> {
        self.call::<String, Vec<CurrencyChainResp>>(
            CallMethod::Post,
            format!("{}/queryChainCurrencyForList", Self::BASE_PATH),
            None,
            None,
        )
        .await
    }

    /// Retrieves a list of user wallets based on the specified query parameters
    ///
    /// This method fetches wallet information for a specific user according to the
    /// provided query parameters.
    ///
    /// # Parameters
    /// * `req` - Query parameters for filtering user wallet information
    ///
    /// # Returns
    /// - `Result<CommonResp<Vec<UserWalletResp>>>`: A list of user wallet information on success
    async fn query_user_wallet_for_list(
        &self,
        req: QueryWalletReq,
    ) -> anyhow::Result<CommonResp<Vec<UserWalletResp>>> {
        self.call(
            CallMethod::Post,
            format!("{}/queryUserWalletForList", Self::BASE_PATH),
            None,
            Some(req),
        )
        .await
    }
}

#[async_trait]
impl<T: Call + Send + Sync> WalletApi for T {}
