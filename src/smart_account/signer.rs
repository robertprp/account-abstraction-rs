use async_trait::async_trait;
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{Signer, Wallet, WalletError},
    types::{transaction::eip712::Eip712, Address, Bytes, H256},
    utils::hash_message,
};
use std::error::Error;

#[async_trait]
pub trait SmartAccountSigner: std::fmt::Debug + Send + Sync {
    // type Error: Error + Send + Sync;
    type Error: Error + Send + Sync + 'static;

    fn get_address(&self) -> Address;

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Bytes, Self::Error>;

    fn sign_hash_data(&self, hash: H256) -> Result<Bytes, Self::Error>;

    async fn sign_typed_eip712_data<T: Eip712 + Send + Sync>(
        &self,
        payload: &T,
    ) -> Result<Bytes, Self::Error>;
}

#[async_trait]
impl SmartAccountSigner for Wallet<SigningKey> {
    type Error = WalletError;

    fn get_address(&self) -> Address {
        self.address()
    }

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Bytes, Self::Error> {
        let message = message.as_ref();
        let message_hash: H256 = hash_message(message);
        let ecdsa_signature: Bytes = self.sign_hash(message_hash)?.to_vec().into();

        Ok(ecdsa_signature)
    }

    fn sign_hash_data(&self, hash: H256) -> Result<Bytes, Self::Error> {
        let signature: Bytes = self.sign_hash(hash)?.to_vec().into();

        Ok(signature)
    }

    async fn sign_typed_eip712_data<T: Eip712 + Send + Sync>(
        &self,
        payload: &T,
    ) -> Result<Bytes, Self::Error> {
        let signature: Bytes = self.sign_typed_data(payload).await?.to_vec().into();

        Ok(signature)
    }
}
