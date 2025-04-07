//! ZK Login authentication tool module
//!
//! This module provides a login authentication function based on zero-knowledge proof, mainly used to generate and verify ZK Login related authentication materials.
//! The module integrates the ZK Login function of the Sui network and supports JWT token verification and transaction signature.
//!
//! # Main functions
//!
//! * Generate ZK Login authentication materials
//! * Use ZK Login for transaction signature
//!
use crate::fastcrypto::encoding::{Base64, Encoding};
use crate::fastcrypto::jwt_utils::parse_and_validate_jwt;
use crate::fastcrypto::traits::EncodeDecodeBase64;
use crate::fastcrypto_zkp::bn254::utils::gen_address_seed;
use crate::fastcrypto_zkp::bn254::zk_login::{ZkLoginInputs, ZkLoginInputsReader};
use crate::shared_crypto::intent::{Intent, IntentMessage};
use crate::sui_types::crypto::{PublicKey, Signature, SuiKeyPair, ZkLoginPublicIdentifier};
use crate::sui_types::signature::GenericSignature;
use crate::sui_types::transaction::TransactionData;
use crate::sui_types::zk_login_authenticator::ZkLoginAuthenticator;

/// Generate the materials required for ZK Login authentication
///
/// # Parameters
///
/// * `jwt_token` - JWT token string
/// * `salt` - Salt value used to generate address seed
/// * `reader` - ZK Login input reader
///
/// # Return value
///
/// Return a tuple containing:
/// * `PublicKey` - Generated public key
/// * `ZkLoginInputs` - Input data required for ZK Login
///
/// # Error
///
/// If JWT token parsing fails or address seed generation fails, an error will be returned
pub fn zklogin_material(
    jwt_token: &str,
    salt: &str,
    reader: ZkLoginInputsReader,
) -> anyhow::Result<(PublicKey, ZkLoginInputs)> {
    // Calculate address_seed and address
    let (sub, aud, _) = parse_and_validate_jwt(jwt_token)?;
    let address_seed = gen_address_seed(salt, "sub", &sub, &aud)?;
    let zk_login_inputs = ZkLoginInputs::from_reader(reader, &address_seed.to_string())?;
    let pk = PublicKey::ZkLogin(ZkLoginPublicIdentifier::new(
        zk_login_inputs.get_iss(),
        zk_login_inputs.get_address_seed(),
    )?);

    Ok((pk, zk_login_inputs))
}

pub fn zklogin_sign_tx_bytes<T: AsRef<str>>(
    max_epoch: u64,
    skp: &SuiKeyPair,
    zk_login_inputs: ZkLoginInputs,
    tx_bytes: T,
) -> anyhow::Result<String> {
    let tx_data: TransactionData = bcs::from_bytes(&Base64::decode(tx_bytes.as_ref())?)?;
    let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data);
    let s = Signature::new_secure(&intent_msg, skp);
    let zk_login_authenticator = ZkLoginAuthenticator::new(zk_login_inputs, max_epoch, s);
    let final_sig: GenericSignature = zk_login_authenticator.into();
    Ok(final_sig.encode_base64())
}
