/// DID Identity Authentication Related API Interfaces
///
/// This trait provides a complete user identity authentication process, including the following steps:
/// 1. Send mobile verification code
/// 2. Mobile number authentication
/// 3. Obtain JWT Token
/// 4. Bind DID wallet
///
/// # Example
///
/// ```rust
/// use fastcrypto::ed25519::Ed25519KeyPair;
/// use fastcrypto::jwt_utils::parse_and_validate_jwt;
/// use fastcrypto::traits::KeyPair;
/// use fastcrypto_zkp::bn254::utils::{gen_address_seed, get_nonce};
/// use fastcrypto_zkp::bn254::zk_login::ZkLoginInputs;
/// use num_bigint::BigUint;
/// use rand::rngs::StdRng;
/// use rand::SeedableRng;
/// use sui_types::base_types::SuiAddress;
/// use sui_types::crypto::{PublicKey, SuiKeyPair, ZkLoginPublicIdentifier};
/// use onechain_wallet_rust_sdk::rpc::DIDApi;
/// use onechain_wallet_rust_sdk::types::common::ACCESS_TOKEN;
/// use onechain_wallet_rust_sdk::types::did::*;
///
/// async fn example<T: DIDApi>(client: &T) {
///     // zk_login example
///     // 1. Send verification code
///     let sms_req = SmsCodeSendReq {
///         mobile: "1234567890".to_string(),
///         mobile_prefix: "86".to_string(),
///         provider: "huione".to_string(),
///         ..Default::default()
///     };
///     let sms_resp = client.send_code(sms_req).await?;
///
///     // 2. Mobile number authentication
///     let auth_req = SmsAuthenticateReq {
///         mobile: "1234567890".to_string(),
///         mobile_prefix: "86".to_string(),
///         sms_code: "123456".to_string(),
///         code: sms_resp.data.unwrap(),
///         provider: "huione".to_string(),
///         ..Default::default()
///     };
///     let auth_resp = client.sms(auth_req).await?;
///
///
///     // 3. Prepare zk_login related data
///     // Maximum epoch
///     let max_epoch = 12;
///     // Temporary private key
///     let skp = SuiKeyPair::Ed25519(Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32])));
///     // JWT random number
///     let jwt_randomness = BigUint::from_bytes_be(&[0; 32]).to_string();
///     let mut eph_pk_bytes = vec![0x00];
///     eph_pk_bytes.extend(skp.public().as_ref());
///     let kp_bigint = BigUint::from_bytes_be(&eph_pk_bytes).to_string();
///     let nonce = get_nonce(&eph_pk_bytes, max_epoch, &jwt_randomness)?;
///
///     let authorize_req = AuthorizeTokenProfileReq {
///         provider: "huione".to_string(),
///         code: auth_resp.code,
///         login_type: "sms".to_string(),
///         nonce,
///     };
///
///     let authorize_resp= client.get_token(authorize_req).await?.get_data()?;
///     client.set_header(ACCESS_TOKEN.to_string(),authorize_resp.access_token.clone());
///
///     // 4. Get zk_proofs
///     let req = ZkProofsReq {
///         max_epoch: max_epoch as i64,
///         jwt_randomness: jwt_randomness.clone(),
///         extended_ephemeral_public_key: kp_bigint.clone(),
///         jwt: authorize_resp.jwt_token.clone(),
///         salt: authorize_resp.salt.clone(),
///         key_claim_name: "sub".to_string(),
///     };
///     // Get zk prover
///     let zk_reader = client.get_zk_proofs(req).await?.get_data()?;
///
///
///     // Calculate address_seed and address
///     let (sub, aud, _) = parse_and_validate_jwt(&authorize_resp.jwt_token)?;
///     let address_seed = gen_address_seed(&authorize_resp.salt, "sub", &sub, &aud)?;
///     let zk_login_inputs = ZkLoginInputs::from_reader(zk_reader, &address_seed.to_string())?;
///     let pk = PublicKey::ZkLogin(ZkLoginPublicIdentifier::new(
///         zk_login_inputs.get_iss(),
///         zk_login_inputs.get_address_seed(),
///     )?);
///     let address = SuiAddress::from(&pk);
///
///     println!("{}",address)
/// }
/// ```
use super::{Call, CallMethod};
use crate::types::{common::CommonResp, did::*};
use anyhow::Result;
use async_trait::async_trait;
use fastcrypto_zkp::bn254::zk_login::ZkLoginInputsReader;

#[async_trait]
pub trait DIDApi: Call {
    const BASE_PATH: &'static str = "/did";
    /// Send mobile verification code
    ///
    /// This is the first step in the user authentication process, sending a verification code to the specified mobile number.
    ///
    /// # Parameters
    /// * `request` - Verification code sending request, including mobile number, country code, etc.
    ///
    /// # Return value
    /// Returns the result of sending the verification code. When successful, the data field contains the verification code identifier for subsequent authentication
    async fn send_code(&self, request: SmsCodeSendReq) -> Result<CommonResp<String>> {
        self.sign_call(
            CallMethod::Post,
            format!("{}/sendCode", Self::BASE_PATH),
            None,
            Some(request),
        )
        .await
    }

    /// Mobile number authentication
    ///
    /// This is the second step in the user authentication process, verifying the mobile verification code entered by the user.
    ///
    /// # Parameters
    /// * `request` - Mobile authentication request, including mobile number, verification code, etc.
    ///
    /// # Return value
    /// Returns the authentication result. When successful, it includes an authentication number used to obtain the JWT Token
    async fn sms(&self, request: SmsAuthenticateReq) -> Result<CommonResp<AuthenticateUserResp>> {
        self.call(
            CallMethod::Post,
            format!("{}/authenticateSms", Self::BASE_PATH),
            None,
            Some(request),
        )
        .await
    }

    /// Obtain authentication JWT Token
    ///
    /// This is the third step in the user authentication process, using the authentication number to obtain a JWT Token.
    ///
    /// # Parameters
    /// * `request` - Token request, including authentication number and other information
    ///
    /// # Return value
    /// Returns the JWT Token and user information
    async fn get_token(
        &self,
        request: AuthorizeTokenProfileReq,
    ) -> Result<CommonResp<AuthorizeTokenProfileResp>> {
        self.call(
            CallMethod::Post,
            format!("{}/getToken", Self::BASE_PATH),
            None,
            Some(request),
        )
        .await
    }

    /// Refresh JWT Token
    ///
    /// When the JWT Token is about to expire, this interface can be used to refresh the Token.
    ///
    /// # Parameters
    /// * `request` - Token refresh request
    ///
    /// # Return value
    /// Returns a new JWT Token and user information
    async fn refresh_jwt_token(
        &self,
        request: AuthorizeRefreshJwtTokenReq,
    ) -> Result<CommonResp<AuthorizeTokenProfileResp>> {
        self.call(
            CallMethod::Post,
            format!("{}/refreshJwtToken", Self::BASE_PATH),
            None,
            Some(request),
        )
        .await
    }

    /// Get user information corresponding to the Token
    ///
    /// Retrieve detailed information about the current user based on the JWT Token.
    ///
    /// # Parameters
    /// * `request` - Token query request
    ///
    /// # Return value
    /// Returns detailed user information, including user ID, nickname, avatar, etc.
    async fn get_token_user_profile(
        &self,
        request: AuthorizeTokenReq,
    ) -> Result<CommonResp<UserTokenProfile>> {
        self.call(
            CallMethod::Post,
            format!("{}/getTokenUserProfile", Self::BASE_PATH),
            None,
            Some(request),
        )
        .await
    }

    /// Get ZK Login proofs
    ///
    /// This is the fourth step in the user authentication process, obtaining the zero-knowledge proof materials required for ZK Login.
    ///
    /// # Parameters
    /// * `request` - ZK proof request, including JWT token, salt, ephemeral public key, and other ZK Login related parameters
    ///
    /// # Return value
    /// Returns the ZK Login proof materials that can be used to generate a ZK Login authenticator for transaction signing
    async fn get_zk_proofs(&self, request: ZkProofsReq) -> Result<CommonResp<ZkLoginInputsReader>> {
        self.call(
            CallMethod::Post,
            format!("{}/getZkProofs", Self::BASE_PATH),
            None,
            Some(request),
        )
        .await
    }
}

#[async_trait]
impl<T: Call + Send + Sync> DIDApi for T {}
