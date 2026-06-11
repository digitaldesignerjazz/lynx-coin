//! Block implementation with embedded signed Transactions.

use crate::transaction::Transaction;
use chrono::{DateTime, Utc};
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    /// Now holds a list of signed transactions instead of a single data string
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: u32,
}

impl Block {
    /// Create the Genesis Block with initial allocation transactions.
    pub fn genesis() -> Self {
        let timestamp = 1781201253;

        // Genesis allocations as special "coinbase-like" transactions
        // In a real system these would be handled more carefully (no signatures needed for genesis)
        let genesis_txs = vec![
            Transaction::new(
                "GENESIS".to_string(),
                "bootstrap_fund_address".to_string(),
                8_000_000,
                Some("Genesis bootstrap allocation".to_string()),
            ),
            Transaction::new(
                "GENESIS".to_string(),
                "development_fund_address".to_string(),
                5_000_000,
                Some("Genesis development allocation".to_string()),
            ),
            Transaction::new(
                "GENESIS".to_string(),
                "community_ecosystem_address".to_string(),
                6_000_000,
                Some("Genesis community allocation".to_string()),
            ),
            Transaction::new(
                "GENESIS".to_string(),
                "strategic_partners_address".to_string(),
                2_000_000,
                Some("Genesis strategic partners allocation".to_string()),
            ),
        ];

        // Note: In production genesis transactions would be specially marked and not require signatures.
        // For now we leave them unsigned as they are part of the trusted genesis state.

        Self {
            index: 0,
            timestamp,
            transactions: genesis_txs,
            prev_hash: "0".repeat(64),
            hash: "0000841180bde10c1913ce0ae4dc7c92fae163c8df2f46f50dd38d76983ab69d".to_string(),
            nonce: 140771,
            difficulty: 4,
        }
    }

    pub fn calculate_hash(&self) -> String {
        // For hashing we serialize the transactions as well
        let tx_data: String = self.transactions
            .iter()
            .map(|tx| format!("{}{}{}{}", tx.from, tx.to, tx.amount, tx.timestamp))
            .collect();

        let block_string = format!(
            "{index}{timestamp}{tx_data}{prev_hash}{nonce}",
            index = self.index,
            timestamp = self.timestamp,
            tx_data = tx_data,
            prev_hash = self.prev_hash,
            nonce = self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(block_string.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Mine a new block containing the given transactions.
    pub fn mine_block(transactions: Vec<Transaction>, prev_hash: String, difficulty: u32) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut nonce = 0u64;
        let target = "0".repeat(difficulty as usize);

        println!("Mining block with {} transaction(s) (difficulty {})...", transactions.len(), difficulty);

        loop {
            let block = Self {
                index: 0,
                timestamp,
                transactions: transactions.clone(),
                prev_hash: prev_hash.clone(),
                hash: String::new(),
                nonce,
                difficulty,
            };

            if block.calculate_hash().starts_with(&target) {
                let hash = block.calculate_hash();
                println!("Block mined! Nonce: {} | Hash: {}", nonce, hash);
                return Self {
                    index: 0,
                    timestamp,
                    transactions,
                    prev_hash,
                    hash,
                    nonce,
                    difficulty,
                };
            }
            nonce += 1;
        }
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(self.timestamp, 0).unwrap_or_default()
    }

    /// Validate all signatures inside the transactions of this block.
    pub fn validate_signatures(&self) -> bool {
        for tx in &self.transactions {
            // Genesis allocation transactions are trusted and unsigned
            if tx.from == "GENESIS" {
                continue;
            }
            match tx.verify() {
                Ok(true) => continue,
                _ => return false,
            }
        }
        true
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Block #{} [{}] ({} txs)", self.index, self.datetime(), self.transactions.len())?;
        writeln!(f, "  Hash:      {}", self.hash)?;
        writeln!(f, "  Prev:      {}", self.prev_hash)?;
        for tx in &self.transactions {
            writeln!(f, "    - {}", tx.summary())?;
        }
        Ok(())
    }
}