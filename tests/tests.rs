use num_bigint::BigUint;
use onechain_wallet_rust_sdk::client::OneChainWalletService;
use onechain_wallet_rust_sdk::fastcrypto::ed25519::Ed25519KeyPair;
use onechain_wallet_rust_sdk::fastcrypto::encoding::{Base64, Encoding};
use onechain_wallet_rust_sdk::fastcrypto::traits::KeyPair;
use onechain_wallet_rust_sdk::fastcrypto_zkp::bn254::utils::get_nonce;
use onechain_wallet_rust_sdk::fastcrypto_zkp::bn254::zk_login::ZkLoginInputs;
use onechain_wallet_rust_sdk::rpc::{DIDApi, TransferApi};
use onechain_wallet_rust_sdk::sui_types::base_types::SuiAddress;
use onechain_wallet_rust_sdk::sui_types::crypto::SuiKeyPair;
use onechain_wallet_rust_sdk::sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use onechain_wallet_rust_sdk::sui_types::transaction::TransactionData;
use onechain_wallet_rust_sdk::types::common::ACCESS_TOKEN;
use onechain_wallet_rust_sdk::types::did::{
    AuthorizeTokenProfileReq, SmsAuthenticateReq, SmsCodeSendReq, ZkProofsReq,
};
use onechain_wallet_rust_sdk::types::transfer::{
    BuildSponsorTxReq, ProxyPayTxReq, TransferOrderReq, TransferOrderTxReq,
};
use onechain_wallet_rust_sdk::utils::zk_login::{zklogin_material, zklogin_sign_tx_bytes};
use rand::{random, thread_rng};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use sui_sdk::SuiClientBuilder;
use sui_sdk::rpc_types::SuiObjectDataOptions;
use sui_sdk::types::base_types::ObjectID;

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletInfo {
    pub pri_key: String,
    pub address: SuiAddress,
    pub did: Option<String>,
    pub merchant_id: String,
    pub keypair: SuiKeyPair,
    pub zk_login_inputs: ZkLoginInputs,
    pub access_token: String,
    pub token_id: String,
    pub max_epoch: u64,
}

#[derive(Debug)]
pub struct UserWallet {
    pub info: WalletInfo,
    pub service: OneChainWalletService,
}

const PRI_KEY: &'static str = "your-base64-encoded-private-key";
async fn init_service() -> anyhow::Result<UserWallet> {
    let mut service =
        OneChainWalletService::new("https://api.example.com", PRI_KEY, "merchant_id", None)?;

    let sui_client = SuiClientBuilder::default()
        .build("https://rpc-devnet.onelabs.cc:443")
        .await?;

    // 1.
    let req = SmsCodeSendReq {
        mobile: "123123123".to_string(),
        mobile_prefix: "855".to_string(),
        provider: "huione".to_string(),
    };

    let resp = service.send_code(req).await?.get_data()?;

    // 2.
    let req = SmsAuthenticateReq {
        mobile_prefix: "855".to_string(),
        mobile: "123123123".to_string(),
        provider: "huione".to_string(),
        sms_code: "000000".to_string(),
        code: resp,
    };

    let rsp = service.sms(req).await?.get_data()?;

    let summary = sui_client
        .governance_api()
        .get_latest_sui_system_state()
        .await?;

    let max_epoch = 30 + summary.epoch;
    let skp = SuiKeyPair::Ed25519(Ed25519KeyPair::generate(&mut thread_rng()));
    let jwt_randomness = random::<i64>().to_string();
    let eph_pk_bytes = skp.to_bytes();
    let nonce = get_nonce(&eph_pk_bytes, max_epoch, &jwt_randomness)?;

    let req = AuthorizeTokenProfileReq {
        provider: "huione".to_string(),
        code: rsp.code,
        login_type: "sms".to_string(),
        nonce,
    };

    // 3.
    let rsp = service.get_token(req).await?.get_data()?;
    let kp_bigint = BigUint::from_bytes_be(&eph_pk_bytes).to_string();
    service.set_header(ACCESS_TOKEN.to_string(), rsp.access_token.clone());
    let access_token = rsp.access_token;
    let token_id = rsp.access_token_profile.jti;

    let req = ZkProofsReq {
        max_epoch: max_epoch as i64,
        jwt_randomness: jwt_randomness.clone(),
        extended_ephemeral_public_key: kp_bigint.clone(),
        jwt: rsp.jwt_token.clone(),
        salt: rsp.salt.clone(),
        key_claim_name: "sub".to_string(),
    };
    // 4. get zk prover
    let reader = service.get_zk_proofs(req).await?.get_data()?;

    let (pk, zk_login_inputs) = zklogin_material(&rsp.jwt_token, &rsp.salt, reader)?;

    let address = SuiAddress::from(&pk);

    Ok(UserWallet {
        info: WalletInfo {
            pri_key: PRI_KEY.to_string(),
            address,
            did: rsp.did,
            merchant_id: "1000000".to_string(),
            keypair: skp,
            zk_login_inputs,
            access_token,
            token_id,
            max_epoch,
        },
        service,
    })
}

#[tokio::test]
async fn transfer_test() {
    let wallet = init_service().await.unwrap();

    let req = TransferOrderReq {
        from_address: wallet.info.address.to_string(),
        to_address: "0x643e921c885d47795e3753803193f37ff70c44a412be9c046de51263f39ba0b5"
            .to_string(),
        coin_type: "OCT".to_string(),
        amount: 0.001.to_string(),
        remark: None,
    };

    let resp = wallet
        .service
        .create_order(req)
        .await
        .unwrap()
        .get_data()
        .unwrap();

    let user_sign = zklogin_sign_tx_bytes(
        wallet.info.max_epoch,
        &wallet.info.keypair,
        wallet.info.zk_login_inputs.clone(),
        resp.raw_transaction.clone(),
    )
    .unwrap();

    let req = TransferOrderTxReq {
        hash: resp.hash,
        tx_bytes: resp.raw_transaction,
        user_sig: user_sign,
    };

    let tx_resp = wallet
        .service
        .send_tx(req)
        .await
        .unwrap()
        .get_data()
        .unwrap();

    println!("{:?}", tx_resp);
}

#[tokio::test]
async fn sponsor_test() {
    let wallet = init_service().await.unwrap();

    let sui_client = SuiClientBuilder::default()
        .build("https://rpc-devnet.onelabs.cc:443")
        .await
        .unwrap();

    let object = sui_client
        .read_api()
        .get_object_with_options(
            ObjectID::from_str(
                "0x3e2f98fe42086f4d6fb41ef0b9d2f6006195c9c3a6e334b6343ba48f55377043",
            )
            .unwrap(),
            SuiObjectDataOptions::new(),
        )
        .await
        .unwrap()
        .data
        .unwrap();

    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder
            .transfer_object(wallet.info.address.clone(), object.object_ref())
            .unwrap();
        builder.finish()
    };
    let data =
        TransactionData::new_programmable(wallet.info.address.clone(), vec![], pt, 100000000, 1000);

    let data_bytes = bcs::to_bytes(&data).unwrap();

    let tx_base64 = Base64::encode(&data_bytes);

    let req = BuildSponsorTxReq {
        address: wallet.info.address.to_string(),
        raw_transaction: tx_base64,
        only_transaction_kind: false,
        gas_budget: Some(0.01.to_string()),
    };

    let resp = wallet
        .service
        .build_sponsor_tx(req)
        .await
        .unwrap()
        .get_data()
        .unwrap();

    let user_sign = zklogin_sign_tx_bytes(
        wallet.info.max_epoch,
        &wallet.info.keypair,
        wallet.info.zk_login_inputs.clone(),
        resp.raw_transaction.clone(),
    )
    .unwrap();

    let req = ProxyPayTxReq {
        user_sig: user_sign,
        tx_bytes: resp.raw_transaction,
        reservation_id: resp.reservation_id,
    };

    let resp = wallet
        .service
        .do_proxy_pay_tx(req)
        .await
        .unwrap()
        .get_data()
        .unwrap();
    println!("{:?}", resp);
}
