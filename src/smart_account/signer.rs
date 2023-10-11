use async_trait::async_trait;
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{Wallet, WalletError},
    types::Bytes,
    utils::hash_message,
};
use std::error::Error;

#[async_trait]
pub trait SmartAccountSigner: std::fmt::Debug + Send + Sync {
    type Error: Error + Send + Sync;

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Bytes, Self::Error>;
}

#[async_trait]
impl SmartAccountSigner for Wallet<SigningKey> {
    type Error = WalletError;

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Bytes, Self::Error> {
        let message = message.as_ref();
        let message_hash = hash_message(message);
        let ecdsa_signature = self.sign_hash(message_hash)?.to_vec().into();

        Ok(ecdsa_signature)
    }
}
