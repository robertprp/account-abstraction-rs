use alloy::{
    primitives::{eip191_hash_message, Address, Bytes, B256},
    signers::{k256::ecdsa::SigningKey, local::LocalSigner, Signer},
};
use async_trait::async_trait;
use std::error::Error;
use thiserror::Error;

#[async_trait]
pub trait SmartAccountSigner: std::fmt::Debug + Send + Sync {
    type Error: Error + Send + Sync + 'static;

    fn get_address(&self) -> Address;

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Bytes, Self::Error>;

    async fn sign_hash_data(&self, hash: &B256) -> Result<Bytes, Self::Error>;

    // async fn sign_typed_eip712_data<T: Send + Sync>(
    //     &self,
    //     payload: &T,
    // ) -> Result<Bytes, Self::Error>;
}

#[async_trait]
impl SmartAccountSigner for LocalSigner<SigningKey> {
    type Error = SignerError;

    fn get_address(&self) -> Address {
        self.address()
    }

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Bytes, Self::Error> {
        let message = message.as_ref();
        let message_hash = eip191_hash_message(message);
        let signature = self
            .sign_hash(&message_hash.into())
            .await
            .map_err(|e| SignerError::HashSigningError(e.to_string()))?;

        Ok(signature.as_bytes().to_vec().into())
    }

    async fn sign_hash_data(&self, hash: &B256) -> Result<Bytes, Self::Error> {
        let signature = self
            .sign_hash(&hash)
            .await
            .map_err(|e| SignerError::HashSigningError(e.to_string()))?;

        Ok(signature.as_bytes().to_vec().into())
    }

    // async fn sign_typed_eip712_data<T: Send + Sync>(
    //     &self,
    //     payload: &T,
    // ) -> Result<Bytes, Self::Error> {
    //     let signature = self.sign_typed_data(payload).await?;
    //     Ok(signature.to_vec().into())
    // }
}

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("hash signing error: {0}")]
    HashSigningError(String),
}
