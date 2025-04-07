//! RPC interface module for OneChain Wallet SDK
//!
//! This module contains all API interface definitions in the OneChain Wallet SDK, including:
//! - did_api: Identity authentication related interfaces
//! - wallet_api: Wallet related interfaces
//! - transfer_api: Transfer related interfaces

mod did_api;
pub use did_api::*;
mod transfer_api;
pub use transfer_api::*;
mod wallet_api;
pub use wallet_api::*;

use crate::types::common::CommonResp;
use crate::utils::rsa_sign::RSASign;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

/// HTTP request method enumeration
///
/// Used to specify the HTTP method type for API requests
pub enum CallMethod {
    /// POST request method
    Post,
    /// GET request method
    Get,
}

#[async_trait]
pub trait Call: RSASign {
    async fn call<Req, Resp>(
        &self,
        method: CallMethod,
        path: String,
        header: Option<HashMap<String, String>>,
        req: Option<Req>,
    ) -> anyhow::Result<CommonResp<Resp>>
    where
        Req: Serialize + Debug + Send,
        Resp: for<'de> Deserialize<'de>;

    async fn sign_call<Req, Resp>(
        &self,
        method: CallMethod,
        path: String,
        header: Option<HashMap<String, String>>,
        req: Option<Req>,
    ) -> anyhow::Result<CommonResp<Resp>>
    where
        Req: Serialize + Debug + Send,
        Resp: for<'de> Deserialize<'de>;
}
