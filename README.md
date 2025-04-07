# OneChain Wallet Rust Sdk

OneChain Wallet Rust SDK is a Rust client SDK for interacting with the OneChain Wallet service.It provides a series of
API interfaces for implementing identity authentication, payment transactions,financial services and other functions.

## Using

Uses Tokio and enables some optional features, so your Cargo.toml could look like this:

```toml
tokio = { version = "1", features = ["full"] }
onechain_wallet_rust_sdk = { git = "https://github.com/one-chain-labs/onechain-wallet-rust-sdk.git" }
sui_sdk = { git = "https://github.com/one-chain-labs/onechain.git", package = "sui-sdk" }
anyhow = "1.0.97"
num-bigint = "0.4.4"
```

## Example

### Huione ZkLogin

**1.Send Sms Code**

```rust
use onechain_wallet_rust_sdk::client::OneChainWalletService;
use onechain_wallet_rust_sdk::rpc::DIDApi;
use onechain_wallet_rust_sdk::types::did::SmsCodeSendReq;

async fn send_sms_code(service: &OneChainWalletService) -> anyhow::Result<String> {
    let req = SmsCodeSendReq {
        mobile: "123123123".to_string(),
        mobile_prefix: "855".to_string(),
        provider: "huione".to_string(),
    };

    // get sms code
    let sms_code = service.send_code(req).await?.get_data()?;
    Ok(sms_code)
}

```

**2.Get Sms Auth Number**

```rust
use onechain_wallet_rust_sdk::client::OneChainWalletService;
use onechain_wallet_rust_sdk::rpc::DIDApi;
use onechain_wallet_rust_sdk::types::did::SmsAuthenticateReq;

async fn sms_auth(service: &OneChainWalletService, sms_code: String) -> anyhow::Result<String> {
    let req = SmsAuthenticateReq {
        mobile_prefix: "855".to_string(),
        mobile: "123123123".to_string(),
        provider: "huione".to_string(),
        sms_code: "000000".to_string(),
        code: sms_code,
    };
    let rsp = service.sms(req).await?.get_data()?;
    Ok(rsp.code)
}

```

**3.Get Zk JWT Token**

```rust
use onechain_wallet_rust_sdk::client::OneChainWalletService;
use onechain_wallet_rust_sdk::fastcrypto::ed25519::Ed25519KeyPair;
use onechain_wallet_rust_sdk::fastcrypto::traits::KeyPair;
use onechain_wallet_rust_sdk::fastcrypto_zkp::bn254::utils::get_nonce;
use onechain_wallet_rust_sdk::rpc::DIDApi;
use onechain_wallet_rust_sdk::sui_types::crypto::SuiKeyPair;
use onechain_wallet_rust_sdk::types::common::ACCESS_TOKEN;
use onechain_wallet_rust_sdk::types::did::{AuthorizeTokenProfileReq, AuthorizeTokenProfileResp};
use rand::{random, thread_rng};
use sui_sdk::SuiClient;

async fn get_jwt_token(
    sui_client: &SuiClient,
    service: &mut OneChainWalletService,
    auth_number: String,
) -> anyhow::Result<(SuiKeyPair, AuthorizeTokenProfileResp)> {
    let summary = sui_client
        .governance_api()
        .get_latest_sui_system_state()
        .await?;

    let max_epoch = 30 + summary.epoch;
    let skp = SuiKeyPair::Ed25519(Ed25519KeyPair::generate(&mut thread_rng()));
    let jwt_randomness = random::<i128>().abs().to_string();
    let eph_pk_bytes = skp.to_bytes();
    let nonce = get_nonce(&eph_pk_bytes, max_epoch, &jwt_randomness)?;

    let req = AuthorizeTokenProfileReq {
        provider: "huione".to_string(),
        code: auth_number,
        login_type: "sms".to_string(),
        nonce,
    };
    let rsp = service.get_token(req).await?.get_data()?;
    // set token header
    service.set_header(ACCESS_TOKEN.to_string(), rsp.access_token.clone());

    Ok((skp, rsp))
}
```

**4.Get ZK Proof**

```rust
use fastcrypto_zkp::bn254::zk_login::ZkLoginInputsReader;
use num_bigint::BigUint;
use onechain_wallet_rust_sdk::client::OneChainWalletService;
use onechain_wallet_rust_sdk::rpc::DIDApi;
use onechain_wallet_rust_sdk::types::did::{AuthorizeTokenProfileResp, ZkProofsReq};
use sui_types::crypto::SuiKeyPair;

async fn get_zk_proof(
    service: &OneChainWalletService,
    max_epoch: i64,
    jwt_randomness: String,
    skp: &SuiKeyPair,
    jwt_token_rsp: &AuthorizeTokenProfileResp,
) -> anyhow::Result<ZkLoginInputsReader> {
    let extended_ephemeral_public_key = BigUint::from_bytes_be(&skp.to_bytes()).to_string();

    let req = ZkProofsReq {
        max_epoch,
        jwt_randomness,
        extended_ephemeral_public_key,
        jwt: jwt_token_rsp.jwt_token.clone(),
        salt: jwt_token_rsp.salt.clone(),
        key_claim_name: "sub".to_string(),
    };
    let reader = service.get_zk_proofs(req).await?.get_data()?;
    Ok(reader)
}

```