//! Simple Wallet / Keystore for Lynx-Coin.
//!
//! Provides key management and convenience methods for creating signed transactions.

use crate::transaction::Transaction;
use ed25519_dalek::SigningKey;
use hex;
use rand::rngs::OsRng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Wallet {
    /// In-memory keystore: address -> (private_key_hex, public_key_hex)
    keys: HashMap<String, (String, String)>,
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    /// Generate and store a new keypair. Returns the address.
    pub fn generate_key(&mut self) -> String {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let priv_hex = hex::encode(signing_key.to_bytes());
        let pub_hex = hex::encode(verifying_key.to_bytes());
        let address = pub_hex.clone();

        self.keys.insert(address.clone(), (priv_hex, pub_hex));
        address
    }

    /// Create and sign a transaction from this wallet.
    pub fn create_signed_transaction(
        &self,
        from: &str,
        to: String,
        amount: u64,
        data: Option<String>,
    ) -> Result<Transaction, String> {
        let (priv_hex, _) = self.keys.get(from).ok_or("Address not found in wallet")?;

        let mut tx = Transaction::new(from.to_string(), to, amount, data);
        tx.sign(priv_hex)?;
        Ok(tx)
    }

    pub fn list_addresses(&self) -> Vec<String> {
        self.keys.keys().cloned().collect()
    }

    /// Load from a simple JSON file (future improvement: encrypted keystore)
    pub fn load_from_file(_path: &str) -> Result<Self, String> {
        // Placeholder for encrypted keystore loading
        Ok(Self::new())
    }
}