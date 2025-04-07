//! Type definition module for OneChain Wallet SDK
//!
//! This module contains all data type definitions in the OneChain Wallet SDK. These types are used to build requests and process responses,
//! ensuring that communication with the OneChain service conforms to expected formats and specifications.
//!
//! # Module Structure
//!
//! * [`common`] - Common request and response types
//!   - Contains the basic request structure [`BaseReq`](common::BaseReq)
//!   - Contains the common response structure [`CommonResp`](common::CommonResp)
//!   - Defines commonly used constants and utility types
//!
//! * [`did`] - Identity authentication related types
//!   - Provides identity verification request types, such as [`AuthorizeTokenReq`](did::AuthorizeTokenReq)
//!   - Contains SMS verification related structures, such as [`SmsCodeSendReq`](did::SmsCodeSendReq)
//!   - Supports request and response types for various authentication methods
//!
//! * [`transfer`] - Transfer related types
//!   - Defines transfer order query structure [`TransferOrderQueryReq`](transfer::TransferOrderQueryReq)
//!   - Contains response types for transfer status and results
//!   - Supports multi-currency, multi-chain transfer operations
//!
//! * [`wallet`] - Wallet related types
//!   - Defines chain and currency information structure [`CurrencyChainResp`](wallet::CurrencyChainResp)
//!   - Contains detailed currency information [`CurrencyInfo`](wallet::CurrencyInfo)
//!   - Supports wallet management and asset query functions
//!
//! # Example
//!
//! Using the common request structure to send a request:
//!
//! ```no_run
//! use onechain_wallet_rust_sdk::types::common::BaseReq;
//! use onechain_wallet_rust_sdk::types::did::SmsCodeSendReq;
//!
//! let req = SmsCodeSendReq {
//!     mobile: "12345678".to_string(),
//!     mobile_prefix: "855".to_string(),
//!     provider: "huione".to_string(),
//!     ..Default::default()
//! };
//!
//! let base_req = BaseReq::new("merchant_id".to_string(), Some(req));
//! ```

pub mod common;
pub mod did;
pub mod transfer;
pub mod wallet;
