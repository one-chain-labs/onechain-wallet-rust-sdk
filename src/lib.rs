//! OneChain Wallet Rust SDK
//!
//! OneChain Wallet Rust SDK is a Rust client SDK for interacting with the OneChain Wallet service.
//! It provides a series of API interfaces for implementing identity authentication, payment transactions,
//! financial services and other functions.
//! The SDK adopts an asynchronous design, supports high-concurrency scenarios,
//! and provides complete type safety and error handling mechanisms.
//!
//! # Features
//!
//! * Complete API support - covers all functional interfaces of OneChain services
//! * Type safety - all API requests and responses have complete type definitions
//! * Asynchronous support - asynchronous runtime based on tokio
//! * Safe and reliable - built-in RSA signature and verification mechanism
//! * Easy to expand - modular design, support for custom expansion
//!
//! # Main modules
//!
//! * [`client`] - Provides the core client implementation of the SDK
//!
//! * [`rpc`] - Contains the definition of all API interfaces
//!
//! * [`types`] - Defines the data structure of all requests and responses
//!
//! * [`utils`] - Provides utility functions
//!
//! # Quick Start
//!
//! To use the OneChain Wallet SDK, you first need to create an instance of [`client::OneChainWalletService`]:
//!
//! ```no_run
//! use onechain_wallet_rust_sdk::client::OneChainWalletService;
//!
//! async fn example() -> anyhow::Result<()> {
//!     let service = OneChainWalletService::new(
//!         "https://api.example.com",  // API service address
//!         "your-base64-encoded-private-key",  // RSAPrivateKey
//!         "merchant_id",
//!         None,
//!     )?;
//!     Ok(())
//! }
//! ```
//!
//! # Usage example
//!
//! ## Send SMS verification code
//!
//! ```no_run
//! use onechain_wallet_rust_sdk::client::OneChainWalletService;
//! use onechain_wallet_rust_sdk::rpc::DIDApi;
//! use onechain_wallet_rust_sdk::types::did::SmsCodeSendReq;
//!
//! async fn send_sms_example() -> anyhow::Result<()> {
//!     let service = OneChainWalletService::new(
//!         "https://api.example.com",
//!         "your-base64-encoded-private-key",
//!         "merchant_id",
//!         None
//!     )?;
//!
//!     let req = SmsCodeSendReq {
//!         mobile: "12345678".to_string(),
//!         mobile_prefix: "855".to_string(),
//!         provider: "huione".to_string(),
//!     };
//!
//!     let resp = service.send_code(req).await?;
//!     println!("{:?}", resp);
//!     Ok(())
//! }
//! ```
//!
pub mod client;
pub mod rpc;
pub mod types;
pub mod utils;

// Re-export dependencies
pub use fastcrypto;
pub use fastcrypto_zkp;
pub use shared_crypto;
pub use sui_types;
