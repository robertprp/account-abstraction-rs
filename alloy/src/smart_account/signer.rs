use alloy::primitives::{Address, Bytes, B256};
use async_trait::async_trait;
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

    fn sign_hash_data(&self, hash: B256) -> Result<Bytes, Self::Error>;

    async fn sign_typed_eip712_data<T: Send + Sync>(
        &self,
        payload: &T,
    ) -> Result<Bytes, Self::Error>;
}

// #[async_trait]
// // impl SmartAccountSigner for Wallet<SigningKey> {
// impl SmartAccountSigner for LocalSigner<SigningKey> {
//     type Error = WalletError;

//     fn get_address(&self) -> Address {
//         self.address()
//     }

//     async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
//         &self,
//         message: S,
//     ) -> Result<Bytes, Self::Error> {
//         let message = message.as_ref();
//         let message_hash: B256 = hash_message(message); // eip191_hash_message?
//         let ecdsa_signature: Bytes = self.sign_hash(message_hash)?.to_vec().into();

//         Ok(ecdsa_signature)
//     }

//     fn sign_hash_data(&self, hash: B256) -> Result<Bytes, Self::Error> {
//         let signature: Bytes = self.sign_hash(hash)?.to_vec().into();

//         Ok(signature)
//     }

//     async fn sign_typed_eip712_data<T: Eip712 + Send + Sync>(
//         &self,
//         payload: &T,
//     ) -> Result<Bytes, Self::Error> {
//         let signature: Bytes = self.sign_typed_data(payload).await?.to_vec().into();

//         Ok(signature)
//     }
// }
