//! RSA signature related tool module
//!
//! This module provides RSA signature related functions, which are used to generate and verify the digital signatures required by OneChain services.
//! The main functions include:
//!
//! - Convert objects to signature strings
//! - Generate RSA private key signatures
//! - Encapsulation of signature tool structures
//!
//! # Main components
//!
//! * [`ToLinkStr`] - Feature, used to convert objects to strings to be signed
//! * [`RSASignUtils`] - RSA signature tool structure, providing signature functions
//!
//! # Example
//!
//! ```no_run
//! use onechain_wallet_rust_sdk::utils::rsa_sign::{RSASignUtils, ToLinkStr};
//! use serde::Serialize;
//!
//! #[derive(Debug, Serialize)]
//! struct MyRequest {
//!     user_id: String,
//!     amount: u64,
//! }
//!
//! impl ToLinkStr for MyRequest {}
//!
//! let private_key = "base64_encoded_private_key";
//! let sign_tool = RSASignUtils::new(private_key).unwrap();
//!
//! let req = MyRequest {
//!     user_id: "123".to_string(),
//!     amount: 100,
//! };
//!
//! // Generate signature
//! let signature = sign_tool.sign(&req, vec![]).unwrap();
//! ```
use base64::{Engine, prelude::BASE64_STANDARD};
use rsa::{
    RsaPrivateKey, pkcs1v15::SigningKey, pkcs8::DecodePrivateKey, sha2::Sha256,
    signature::RandomizedSigner,
};
use serde::Serialize;
use signature::SignatureEncoding;
use std::collections::HashSet;
use std::fmt::Debug;

pub trait ToLinkStr: Serialize + Debug {
    fn to_link_str(&self, ignore_fields: Vec<&str>) -> anyhow::Result<String> {
        let ignore_fields = ignore_fields.into_iter().collect::<HashSet<_>>();
        let jv = serde_json::to_value(self)?;
        match jv {
            serde_json::Value::Object(m) => {
                let mut keys = m.keys().collect::<Vec<_>>();
                keys.sort();
                let v = keys
                    .into_iter()
                    .filter_map(|k| {
                        if ignore_fields.contains(&k.as_str()) {
                            None
                        } else {
                            match &m[k] {
                                serde_json::Value::Bool(v) => Some(format!("{}={}", k, v)),
                                serde_json::Value::Number(v) => Some(format!("{}={}", k, v)),
                                serde_json::Value::String(v) => {
                                    if v.is_empty() {
                                        None
                                    } else {
                                        Some(format!("{}={}", k, v))
                                    }
                                }
                                serde_json::Value::Object(v) => {
                                    // it's json value, so it's safe to unwrap
                                    Some(format!("{}={}", k, serde_json::to_string(v).unwrap()))
                                }
                                _ => None,
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                let s = v.join("&");
                Ok(s)
            }
            _ => {
                anyhow::bail!("wrong callback req obj: {:?}", self);
            }
        }
    }
}

impl<T: Serialize + Debug> ToLinkStr for T {}

/// RSA signature interface
///
/// This trait defines the basic interface of RSA signature.
/// The type that implements this trait needs to provide a sign method.
pub trait RSASign {
    /// RSA sign the data
    ///
    /// # Parameters
    ///
    /// * `obj` - the data object to be signed, must implement Serialize and Debug traits
    /// * `ignore_fields` - a list of fields to be ignored when signing
    ///
    /// # Return value
    ///
    /// Returns the Base64-encoded signature string. If an error occurs during the signing process, an error is returned
    fn sign<T: Serialize + Debug>(
        &self,
        obj: &T,
        ignore_fields: Vec<&str>,
    ) -> anyhow::Result<String>;
}

/// RSA signature tool structure
///
/// This structure provides a specific implementation of RSA signature, using SHA256 as the hash algorithm.
///
/// # Example
///
/// ```rust
/// use onechain_wallet_rust_sdk::utils::rsa_sign::RSASignUtils;
/// use serde::Serialize;
///
/// #[derive(Debug, Serialize)]
/// struct MyData {
///     field1: String,
///     field2: i32,
/// }
///
/// let private_key = "base64_encoded_private_key";
/// let rsa_utils = RSASignUtils::new(private_key).unwrap();
///
/// let data = MyData {
///     field1: "value1".to_string(),
///     field2: 42,
/// };
///
/// let signature = rsa_utils.sign(&data, vec![]).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct RSASignUtils {
    sk: SigningKey<Sha256>,
}

impl RSASignUtils {
    /// Create a new RSASignUtils instance
    ///
    /// # Parameters
    ///
    /// * `b64der` - Base64-encoded PKCS8 format RSA private key
    ///
    /// # Return value
    ///
    /// Returns the RSASignUtils instance, or an error if private key parsing fails
    pub fn new(b64der: &str) -> anyhow::Result<Self> {
        let der = BASE64_STANDARD.decode(b64der)?;
        let rpk = RsaPrivateKey::from_pkcs8_der(&der)?;
        let sk = SigningKey::<Sha256>::new(rpk);
        Ok(Self { sk })
    }

    /// RSA sign the data
    ///
    /// # Parameters
    ///
    /// * `obj` - the data object to be signed
    /// * `ignore_fields` - a list of fields to be ignored when signing
    ///
    /// # Return value
    ///
    /// Returns the Base64-encoded signature string
    pub fn sign<T: Serialize + Debug>(
        &self,
        obj: &T,
        ignore_fields: Vec<&str>,
    ) -> anyhow::Result<String> {
        let link_str = obj.to_link_str(ignore_fields)?;
        let mut rng = rand::thread_rng();
        let sig = self.sk.try_sign_with_rng(&mut rng, link_str.as_bytes())?;
        let sig_byt = sig.to_vec();
        Ok(BASE64_STANDARD.encode(sig_byt))
    }
}

#[cfg(test)]
mod test {
    const PUB_KEY: &'static str = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAr43KS8cko41MYEyDAlwqm3t9JRmBtTQQnm7l+RzrBCvPODRmpZGNhpO2MUgVFYdWkHlt/zTEGAqkhDUXkkwpeHebB9zWhTbhDGEdohW5T82MtdihNGgemoeNpC/eTt46o/5nqHzbe84CNhefEQdVMmYJcnX2Ma/g5VzFXOjOM7/ThE02L4TIMAjsFhapXRMcxZ4i0D2Xn0HVtl2uEURdXdQHnoAKjoGHukV4S/olMw8B6u2N0TpjJt9ORKCvIBYvsXgyVVcUzMLmUDIiS+RbhqZ60R9bTDeYSzm8ej/WgRM0ap6U89DDUvtEN1atb00rKqW+aU/ob0FU83Q2LeLq9QIDAQAB";
    const PRI_KEY: &'static str = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCvjcpLxySjjUxgTIMCXCqbe30lGYG1NBCebuX5HOsEK884NGalkY2Gk7YxSBUVh1aQeW3/NMQYCqSENReSTCl4d5sH3NaFNuEMYR2iFblPzYy12KE0aB6ah42kL95O3jqj/meofNt7zgI2F58RB1UyZglydfYxr+DlXMVc6M4zv9OETTYvhMgwCOwWFqldExzFniLQPZefQdW2Xa4RRF1d1AeegAqOgYe6RXhL+iUzDwHq7Y3ROmMm305EoK8gFi+xeDJVVxTMwuZQMiJL5FuGpnrRH1tMN5hLObx6P9aBEzRqnpTz0MNS+0Q3Vq1vTSsqpb5pT+hvQVTzdDYt4ur1AgMBAAECggEAAgw0WEo3pZ3evFX12KsO1L27kvTHWdIo7uS6QSBSy7uEOkBRE+fjuNshpZ5eDSmFG2TfM3D/+kKrO7pmzrLCJ1xIEspnpHL/2dz4s18mWqDxAoMif1+QGq2dO8MuCDbmg+rkdlmmeuGisveuI3FsmIycbHSlyKSVifdZMfyqUxB3ysLl4SQLxoZ2x4NL9e/Jj8NlKKgLZeXgqp4/ojh3IUGwHIYnz2PVm+K55wbq55E61p3yyd+09kIOajqLx+d6CsfNY9MhOXdl9W7vZEGKeQ1HuCQ9muwEAmO32yELQC39t4Q7GuPD+0shTMmDs6QsUXzZC/XfqBd3xPBlwkvIgQKBgQDXiUNMfctRYQuFkLR3Ux+rpAxCOxLLdGjoSyVJaZzeAci5U28R4CbVeVV1HeRmy+x2kwe1YD/7x6qCxQUlRnDYATcZJf09YrrrZFXPQilCTqi2RdWy1Zq0M9sEhFWsJL4QF0fF/puXwXVbRB/uVMbH/jyT5wFNxbmmNxWixtK41QKBgQDQgvbxxDdc+WSWnAj0uTsiDloewmeueh/IdnGTPSx5qfF931VeWl4waOqhI8N6sDEYhvMa8+XjDdJZ08YdPh9bPQIhNCcEbL2u9SEt2VZ7nx/oVPQCyBIHsXaOoPtPH68qnTlSPhDajZALhPQVQwpxizmTfVuyi/hZG1OsYgB5oQKBgQCGva6uwO074JkdVIsdFX/1A0cOmHN1cT6sCV4z+KwyNZdQFBKZcDGWvpVn89n3UYBv2Ba3koYtVnMH8Tb4SIL+5jOVqyQXHgOQaFckjE3Sv+3ElP+1Hsfp44kF19zfEtEmqgcahcKrKiu9dGcpzSG/oPYp1/3+qp8Wg9Uov3a4SQKBgDT17s89rWo6FiiC/WtbWP+vcYh6jGcusb/zBaoGUbOdTK9R+Jb8kQvuuhmvwcj5056NOFZSOMPREOqr9Zgb3U8JUe8pFffzvsIflQvWNjc0FaCnY0sJkjrOAnT7wpk4TP+f651OEm3QoxOp820rGA369ObXYmEZWD0ZycjxI3nBAoGANwAhsbfdNuzl6wHHzjRu4kmZJhOSnK1/aHlBpXc/ynBR+BVBghHseKm50azOp6Tx3D19zgViaXGWiH/x3wr2qab5Jy33njx3VUu1r0lug9PoOfzhZ3HqfMT7hAqnuZDn4Ey/t4fzIn38o9yPd8tvZkLnMqZPGR6bE4kLYfforAs=";

    use super::RSASignUtils;
    use crate::types::common::BaseReq;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn sign() {
        let rsa_client = RSASignUtils::new(PRI_KEY).unwrap();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let base_req: BaseReq<String> = BaseReq {
            timestamp,
            merchant_id: "1000000".to_string(),
            merchant_sign: "".to_string(),
            body: None,
        };

        let sign = rsa_client.sign(&base_req, vec!["merchantSign"]).unwrap();

        println!("{:?}", sign);
    }
}
