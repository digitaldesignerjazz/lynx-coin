//! Lynx-Coin Core v0.2
//!
//! Full blockchain with genesis + ed25519 transaction signatures.

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hex;
use rand::rngs::OsRng;
use rand_core::OsRng as CoreOsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs;

mod transaction;

// Re-export for convenience
pub use transaction::Transaction;

/// Block struct (from v0.1, kept for compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: u32,
}

impl Block {
    pub fn genesis() -> Self {
        let timestamp = 1781201253;
        let data = serde_json::json!({
            "message": "Lynx-Coin Genesis Block",
            "allocations": {
                "bootstrap_fund": 8_000_000,
                "development_fund": 5_000_000,
                "community_ecosystem": 6_000_000,
                "strategic_partners": 2_000_000
            },
            "total_supply": 21_000_000,
            "note": "Immutable starting state for Lynx-Coin mesh-native economy."
        }).to_string();

        Self {
            index: 0,
            timestamp,
            data,
            prev_hash: "0".repeat(64),
            hash: "0000841180bde10c1913ce0ae4dc7c92fae163c8df2f46f50dd38d76983ab69d".to_string(),
            nonce: 140771,
            difficulty: 4,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let block_string = format!(
            "{index}{timestamp}{data}{prev_hash}{nonce}",
            index = self.index,
            timestamp = self.timestamp,
            data = self.data,
            prev_hash = self.prev_hash,
            nonce = self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_string.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn mine_block(data: String, prev_hash: String, difficulty: u32) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut nonce = 0u64;
        let target = "0".repeat(difficulty as usize);

        println!("Mining new block (difficulty {})...", difficulty);

        loop {
            let block = Self {
                index: 0,
                timestamp,
                data: data.clone(),
                prev_hash: prev_hash.clone(),
                hash: String::new(),
                nonce,
                difficulty,
            };
            let hash = block.calculate_hash();
            if hash.starts_with(&target) {
                println!("Mined! Nonce: {} | Hash: {}", nonce, hash);
                return Self { index: 0, timestamp, data, prev_hash, hash, nonce, difficulty };
            }
            nonce += 1;
            if nonce % 100_000 == 0 { print!("."); }
        }
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(self.timestamp, 0).unwrap_or_default()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Block #{} [{} ]", self.index, self.datetime())?;
        writeln!(f, "  Hash:      {}", self.hash)?;
        writeln!(f, "  Prev Hash: {}", self.prev_hash)?;
        writeln!(f, "  Nonce:     {} (diff: {})", self.nonce, self.difficulty)?;
        writeln!(f, "  Data:      {}", self.data)?;
        Ok(())
    }
}

/// Blockchain container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { chain: vec![Block::genesis()] }
    }

    pub fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, mut block: Block) {
        block.index = self.chain.len() as u64;
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i-1];
            if current.prev_hash != previous.hash { return false; }
            if current.hash != current.calculate_hash() { return false; }
            let target = "0".repeat(current.difficulty as usize);
            if !current.hash.starts_with(&target) { return false; }
        }
        true
    }

    pub fn mine_and_add(&mut self, data: String, difficulty: u32) {
        let prev = self.latest_block();
        let mut new_block = Block::mine_block(data, prev.hash.clone(), difficulty);
        new_block.index = self.chain.len() as u64;
        new_block.hash = new_block.calculate_hash();
        self.chain.push(new_block);
    }
}

// ==================== CLI ====================

#[derive(Parser)]
#[command(name = "lynx-coin")]
#[command(about = "Lynx-Coin v0.2 - Genesis blockchain + ed25519 transaction signatures")]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Show,
    Mine {
        data: String,
        #[arg(short, long, default_value_t = 4)]
        difficulty: u32,
    },
    Validate,

    /// Generate new Ed25519 keypair
    Keygen,

    /// Create unsigned transaction (outputs JSON)
    CreateTx {
        from: String,
        to: String,
        amount: u64,
        #[arg(long)]
        data: Option<String>,
    },

    /// Sign a transaction JSON with private key
    SignTx {
        tx: String,
        private_key: String,
    },

    /// Verify a signed transaction
    VerifyTx {
        tx: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut chain = Blockchain::new();

    match cli.command {
        Commands::Show => {
            println!("\n=== Lynx-Coin Blockchain (v0.2) ===\n");
            for block in &chain.chain {
                println!("{}", block);
                println!("────────────────────────────────────────");
            }
            println!("Length: {} | Valid: {}", chain.chain.len(), chain.is_valid());
        }
        Commands::Mine { data, difficulty } => {
            chain.mine_and_add(data, difficulty);
            println!("\nBlock mined and added successfully!");
        }
        Commands::Validate => {
            println!("Chain valid: {}", chain.is_valid());
        }

        Commands::Keygen => {
            let (priv_key, pub_key, address) = Transaction::generate_keypair();
            println!("New Ed25519 Keypair:\n");
            println!("Private Key (SECRET): {}", priv_key);
            println!("Public Key:           {}", pub_key);
            println!("Address:              {}", address);
        }

        Commands::CreateTx { from, to, amount, data } => {
            let tx = Transaction::new(from, to, amount, data);
            println!("{}", serde_json::to_string_pretty(&tx).unwrap());
        }

        Commands::SignTx { tx, private_key } => {
            let mut tx: Transaction = if tx.starts_with('{') {
                serde_json::from_str(&tx).unwrap()
            } else {
                serde_json::from_str(&fs::read_to_string(&tx).unwrap()).unwrap()
            };
            match tx.sign(&private_key) {
                Ok(()) => println!("Signed:\n{}", serde_json::to_string_pretty(&tx).unwrap()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Commands::VerifyTx { tx } => {
            let tx: Transaction = if tx.starts_with('{') {
                serde_json::from_str(&tx).unwrap()
            } else {
                serde_json::from_str(&fs::read_to_string(&tx).unwrap()).unwrap()
            };
            match tx.verify() {
                Ok(true) => println!("✓ Signature VALID"),
                Ok(false) => println!("✗ Signature INVALID"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
