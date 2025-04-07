use serde::{Deserialize, Serialize};

/// Request for obtaining authorization token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeTokenProfileReq {
    /// Authorization code
    /// Required field, cannot be empty
    pub code: String,
    /// Random string
    /// Required field, cannot be empty
    pub nonce: String,
    /// Authentication client type, default is huione
    pub provider: String,
    /// Login type, for example: sms
    pub login_type: String,
}

/// Mobile phone authorization request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsAuthenticateReq {
    /// Phone number prefix
    /// Required field, cannot be empty
    pub mobile_prefix: String,
    /// Mobile phone number
    /// Required field, cannot be empty
    pub mobile: String,
    /// Code returned after successfully sending verification code
    /// Required field, cannot be empty
    pub code: String,
    /// SMS verification code
    /// Required field, cannot be empty
    pub sms_code: String,
    /// Access channel: huione
    pub provider: String,
}

/// Send SMS verification code request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsCodeSendReq {
    /// Mobile phone number
    /// Required field, cannot be empty
    pub mobile: String,
    /// Mobile phone number prefix
    /// Required field, cannot be empty
    pub mobile_prefix: String,
    /// Channel, default is huione
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateUserResp {
    /// Authentication number
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenProfile {
    /// Issuer
    pub iss: String,
    /// Authorized party
    pub azp: String,
    /// Audience
    pub aud: String,
    /// Subject
    pub sub: String,
    /// Random value
    pub nonce: String,
    /// Token effective time
    pub nbf: i64,
    /// Token issuance time
    pub iat: i64,
    /// Token expiration time
    pub exp: i64,
    /// Token unique identifier
    pub jti: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeTokenProfileResp {
    /// Token information
    pub access_token_profile: AccessTokenProfile,
    /// Access Token KEY=ACCESS_TOKEN
    pub access_token: String,
    /// JWT-Token
    pub jwt_token: String,
    /// Whether payment password is set
    pub setting_pay_password: bool,
    /// Avatar
    pub avatar_url: Option<String>,
    /// Name
    pub nickname: Option<String>,
    /// Did
    pub did: Option<String>,
    /// User salt value
    pub salt: String,
    /// Whether anonymous
    pub anonymous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTokenProfile {
    /// Expiration time
    pub expire_time: i64,
    /// Username
    pub user_name: String,
    /// Avatar
    pub avatar: String,
    /// ID
    pub id: i64,
    /// Channel user number
    pub channel_user_no: String,
    /// User number
    pub user_no: String,
    /// Access token
    pub access_token: String,
    /// Provider hc
    pub provider: String,
    /// DID
    pub did: String,
}

/// Request for obtaining ZK proof
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZkProofsReq {
    /// Maximum epoch
    /// Required field, cannot be empty
    pub max_epoch: i64,
    /// JWT randomness
    /// Required field, cannot be empty
    pub jwt_randomness: String,
    /// Temporary public key
    /// Required field, cannot be empty
    pub extended_ephemeral_public_key: String,
    /// JWT-token information
    /// Required field, cannot be empty
    pub jwt: String,
    /// Salt value
    /// Required field, cannot be empty
    pub salt: String,
    /// JWT-sub information, user number
    /// Required field, cannot be empty
    pub key_claim_name: String,
}

/// Refresh token request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshJwtTokenReq {
    /// Random string
    /// Required field, cannot be empty
    pub nonce: String,
}
