//! Transaction with ed25519 signatures for Lynx-Coin.
//!
//! Every value-transferring action on the network should be represented
//! as a signed Transaction. This module provides creation, signing,
//! and verification using the ed25519-dalek crate.

use chrono::Utc;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hex;
use rand::rngs::OsRng;
use rand_core::OsRng as CoreOsRng; // for ed25519-dalek 2.x compatibility
use serde::{Deserialize, Serialize};
use std::fmt;

/// A signed transaction on the Lynx-Coin network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Sender address (hex-encoded public key for simplicity in v0.2)
    pub from: String,
    /// Recipient address
    pub to: String,
    /// Amount in smallest unit (e.g. 1 LYNX = 1_000_000_000 units)
    pub amount: u64,
    /// Unix timestamp to help prevent replay attacks
    pub timestamp: i64,
    /// Optional memo or payload (e.g. agent message, invoice id)
    pub data: Option<String>,
    /// Ed25519 signature (hex encoded)
    pub signature: Option<String>,
    /// Public key of the signer (hex encoded) - needed for verification
    pub public_key: Option<String>,
}

impl Transaction {
    /// Create a new unsigned transaction.
    pub fn new(from: String, to: String, amount: u64, data: Option<String>) -> Self {
        Self {
            from,
            to,
            amount,
            timestamp: Utc::now().timestamp(),
            data,
            signature: None,
            public_key: None,
        }
    }

    /// Generate a new random Ed25519 keypair.
    /// Returns (private_key_hex, public_key_hex, address)
    pub fn generate_keypair() -> (String, String, String) {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let private_hex = hex::encode(signing_key.to_bytes());
        let public_hex = hex::encode(verifying_key.to_bytes());
        // Simple address = first 20 chars of public key hex (or full for v0.2)
        let address = public_hex.clone();

        (private_hex, public_hex, address)
    }

    /// Sign this transaction with the given private key (hex).
    /// Returns the transaction with signature and public_key filled.
    pub fn sign(&mut self, private_key_hex: &str) -> Result<(), String> {
        let private_bytes = hex::decode(private_key_hex)
            .map_err(|e| format!("Invalid private key hex: {}", e))?;

        if private_bytes.len() != 32 {
            return Err("Private key must be 32 bytes".to_string());
        }

        let signing_key = SigningKey::from_bytes(
            &private_bytes.try_into().map_err(|_| "Invalid key length".to_string())?,
        );

        let verifying_key = signing_key.verifying_key();
        let message = self.canonical_message();

        let signature = signing_key.sign(&message);

        self.signature = Some(hex::encode(signature.to_bytes()));
        self.public_key = Some(hex::encode(verifying_key.to_bytes()));

        Ok(())
    }

    /// Verify the signature on this transaction.
    pub fn verify(&self) -> Result<bool, String> {
        let sig_hex = self.signature.as_ref().ok_or("Transaction is not signed")?;
        let pk_hex = self.public_key.as_ref().ok_or("Missing public key")?;

        let sig_bytes = hex::decode(sig_hex).map_err(|e| format!("Bad signature hex: {}", e))?;
        let pk_bytes = hex::decode(pk_hex).map_err(|e| format!("Bad public key hex: {}", e))?;

        if sig_bytes.len() != 64 || pk_bytes.len() != 32 {
            return Err("Invalid signature or public key length".to_string());
        }

        let signature = Signature::from_bytes(
            &sig_bytes.try_into().map_err(|_| "Invalid signature length".to_string())?,
        );
        let verifying_key = VerifyingKey::from_bytes(
            &pk_bytes.try_into().map_err(|_| "Invalid public key length".to_string())?,
        ).map_err(|e| format!("Invalid verifying key: {}", e))?;

        let message = self.canonical_message();

        match verifying_key.verify(&message, &signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Create the canonical byte representation used for signing.
    /// This prevents malleability and replay attacks when combined with timestamp.
    fn canonical_message(&self) -> Vec<u8> {
        let mut msg = Vec::new();
        msg.extend_from_slice(self.from.as_bytes());
        msg.extend_from_slice(self.to.as_bytes());
        msg.extend_from_slice(&self.amount.to_le_bytes());
        msg.extend_from_slice(&self.timestamp.to_le_bytes());

        if let Some(ref data) = self.data {
            msg.extend_from_slice(data.as_bytes());
        }
        msg
    }

    /// Human friendly summary
    pub fn summary(&self) -> String {
        format!(
            "{} LYNX from {} → {} (ts: {}, signed: {})",
            self.amount,
            &self.from[..std::cmp::min(16, self.from.len())],
            &self.to[..std::cmp::min(16, self.to.len())],
            self.timestamp,
            self.signature.is_some()
        )
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Transaction {}", self.summary())?;
        if let Some(ref sig) = self.signature {
            writeln!(f, "  Signature: {}...", &sig[..std::cmp::min(16, sig.len())])?;
        }
        if let Some(ref data) = self.data {
            writeln!(f, "  Data: {}", data)?;
        }
        Ok(())
    }
}