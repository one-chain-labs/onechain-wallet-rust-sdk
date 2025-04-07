//! Client module for OneChain Wallet SDK
//!
//! This module provides the main client implementation for OneChain Wallet SDK, including:
//! - HTTP client configuration and management
//! - RSA signature functionality
//! - API call interfaces

use crate::types::common::{BaseReq, CommonResp};
use crate::{
    rpc::{Call, CallMethod},
    utils::rsa_sign::{RSASign, RSASignUtils},
};
use anyhow::Ok;
use async_trait::async_trait;
use reqwest::Url;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::{Jitter, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};

/// OneChain Wallet service client
///
/// This struct provides the main functionality for interacting with OneChain Wallet service, including:
/// - HTTP request sending and processing
/// - RSA signature generation
/// - File upload
///
/// # Example
///
/// ```rust
/// use onechain_wallet_rust_sdk::client::OneChainWalletService;
/// use onechain_wallet_rust_sdk::rpc::Call;
///
/// async fn example() -> anyhow::Result<()> {
///     let client = OneChainWalletService::new(
///         "https://api.example.com",
///         "your-base64-encoded-private-key",
///         "merchant_id",
///         None
///     )?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct OneChainWalletService {
    merchant_id: String,
    /// Base URL of the server
    url: Url,
    /// RSA signature utility
    rsa_client: RSASignUtils,
    /// HTTP client
    http_client: Arc<ClientWithMiddleware>,
    global_headers: HashMap<String, String>,
}

impl OneChainWalletService {
    /// Creates a new OneChainWalletService instance
    ///
    /// # Parameters
    ///
    /// * `host` - Base URL of the service
    /// * `b64der` - Base64 encoded RSA private key
    /// * `merchant_id` - Merchant ID
    /// * `client` - Optional HTTP client, if not provided, a new client will be created with default configuration
    ///
    /// # Returns
    ///
    /// Returns a OneChainService instance that implements the Call trait
    pub fn new(
        host: &str,
        b64der: &str,
        merchant_id: &str,
        client: Option<Arc<ClientWithMiddleware>>,
    ) -> anyhow::Result<Self> {
        let client = if let Some(client) = client {
            client
        } else {
            Arc::new(get_rest_client())
        };

        Ok(Self {
            merchant_id: merchant_id.to_string(),
            url: Url::parse(host)?,
            rsa_client: RSASignUtils::new(b64der)?,
            http_client: client,
            global_headers: HashMap::new(),
        })
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.global_headers.insert(key, value);
    }

    fn get_request_builder(
        &self,
        url: Url,
        method: CallMethod,
        header: Option<HashMap<String, String>>,
    ) -> RequestBuilder {
        let mut request = match method {
            CallMethod::Get => self.http_client.get(url),
            CallMethod::Post => self.http_client.post(url),
        };
        // Add request headers
        if let Some(headers) = header {
            for (key, value) in headers {
                request = request.header(&key, value);
            }
        }

        for (key, value) in self.global_headers.iter() {
            request = request.header(key, value);
        }

        request
    }
}

impl RSASign for OneChainWalletService {
    /// Signs data using RSA
    ///
    /// # Parameters
    ///
    /// * `obj` - The data object to be signed
    /// * `ignore_fields` - List of fields to ignore during signing
    ///
    /// # Returns
    ///
    /// Returns a Base64 encoded signature string
    fn sign<T: Serialize + Debug>(
        &self,
        obj: &T,
        ignore_fields: Vec<&str>,
    ) -> anyhow::Result<String> {
        self.rsa_client.sign(obj, ignore_fields)
    }
}

#[async_trait]
impl Call for OneChainWalletService {
    /// Sends API request
    ///
    /// # Parameters
    ///
    /// * `method` - HTTP request method
    /// * `path` - Request path
    /// * `header` - Optional request headers
    /// * `req` - Request data
    ///
    /// # Returns
    ///
    /// Returns API response result
    async fn call<Req, Resp>(
        &self,
        method: CallMethod,
        path: String,
        header: Option<HashMap<String, String>>,
        req: Option<Req>,
    ) -> anyhow::Result<CommonResp<Resp>>
    where
        Req: Serialize + Debug + Send,
        Resp: for<'de> Deserialize<'de>,
    {
        let url = self.url.join(&path)?;

        let request = self.get_request_builder(url, method, header);
        // Send request and process response
        let response = if let Some(req) = req {
            request.json(&req).send().await?
        } else {
            request.send().await?
        };

        // Parse response body
        let result = response.json::<CommonResp<Resp>>().await?;
        Ok(result)
    }

    async fn sign_call<Req, Resp>(
        &self,
        method: CallMethod,
        path: String,
        header: Option<HashMap<String, String>>,
        req: Option<Req>,
    ) -> anyhow::Result<CommonResp<Resp>>
    where
        Req: Serialize + Debug + Send,
        Resp: for<'de> Deserialize<'de>,
    {
        let mut base_req = BaseReq::new(self.merchant_id.clone(), req);
        let merchant_sign = self.sign(&base_req, vec!["merchantSign"])?;
        base_req.merchant_sign = merchant_sign;

        self.call(method, path, header, Some(base_req)).await
    }
}

/// Creates default HTTP client
///
/// Returns an HTTP client configured with tracing middleware
pub fn get_rest_client() -> ClientWithMiddleware {
    let retry_policy = ExponentialBackoff::builder()
        .retry_bounds(Duration::from_secs(1), Duration::from_secs(11))
        .jitter(Jitter::Bounded)
        .base(2)
        .build_with_total_retry_duration(Duration::from_millis(2_3721));
    ClientBuilder::new(reqwest::Client::new())
        .with(TracingMiddleware::default())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
}
