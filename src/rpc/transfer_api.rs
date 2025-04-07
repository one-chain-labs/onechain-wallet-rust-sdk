use super::{Call, CallMethod};
use crate::types::{
    common::{CommonResp, CreateOrderResp, PageResult},
    transfer::*,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransferApi: Call {
    const BASE_PATH: &'static str = "/transfer";
    /// Create a transfer order
    ///
    /// # Arguments
    /// * `req` - The transfer order request containing order details
    ///
    /// # Returns
    /// * `Result<CommonResp<CreateOrderResp>>` - The created order response
    async fn create_order(&self, req: TransferOrderReq) -> Result<CommonResp<CreateOrderResp>> {
        self.call(
            CallMethod::Post,
            format!("{}/createOrder", Self::BASE_PATH),
            None,
            Some(req),
        )
        .await
    }

    /// Execute a transfer transaction
    ///
    /// # Arguments
    /// * `req` - The transfer transaction request
    ///
    /// # Returns
    /// * `Result<CommonResp<TransferOrderTxResponse>>` - The transaction execution response
    async fn send_tx(
        &self,
        req: TransferOrderTxReq,
    ) -> Result<CommonResp<TransferOrderTxResponse>> {
        self.call(
            CallMethod::Post,
            format!("{}/sendTx", Self::BASE_PATH),
            None,
            Some(req),
        )
        .await
    }

    /// Query transfer orders with pagination
    ///
    /// # Arguments
    /// * `req` - The pagination query request
    ///
    /// # Returns
    /// * `Result<CommonResp<PageResult<TransferOrderResp>>>` - The paginated list of transfer orders
    async fn page_list(
        &self,
        req: TransferOrderQueryPageReq,
    ) -> Result<CommonResp<PageResult<TransferOrderResp>>> {
        self.call(
            CallMethod::Post,
            format!("{}/pageList", Self::BASE_PATH),
            None,
            Some(req),
        )
        .await
    }

    /// Query a specific transfer order
    ///
    /// # Arguments
    /// * `req` - The order query request
    ///
    /// # Returns
    /// * `Result<CommonResp<TransferOrderResp>>` - The transfer order details
    async fn query_order(
        &self,
        req: TransferOrderQueryReq,
    ) -> Result<CommonResp<TransferOrderResp>> {
        self.call(
            CallMethod::Post,
            format!("{}/queryOrder", Self::BASE_PATH),
            None,
            Some(req),
        )
        .await
    }

    /// Build a sponsored transaction
    ///
    /// # Arguments
    /// * `req` - The sponsored transaction build request
    ///
    /// # Returns
    /// * `Result<CommonResp<GasTxBuilderResponse>>` - The built transaction response
    async fn build_sponsor_tx(
        &self,
        req: BuildSponsorTxReq,
    ) -> Result<CommonResp<GasTxBuilderResponse>> {
        self.call(
            CallMethod::Post,
            format!("{}/buildSponsorTransaction", Self::BASE_PATH),
            None,
            Some(req),
        )
        .await
    }

    /// Send a proxy payment transaction
    ///
    /// # Arguments
    /// * `req` - The proxy payment transaction request
    ///
    /// # Returns
    /// * `Result<CommonResp<ProxyPayTxResp>>` - The proxy payment transaction response
    async fn do_proxy_pay_tx(&self, req: ProxyPayTxReq) -> Result<CommonResp<ProxyPayTxResp>> {
        self.call(
            CallMethod::Post,
            format!("{}/doProxyPayTx", Self::BASE_PATH),
            None,
            Some(req),
        )
        .await
    }
}

#[async_trait]
impl<T: Call + Send + Sync> TransferApi for T {}
