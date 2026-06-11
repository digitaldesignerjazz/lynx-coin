//! Lynx-Coin Core
//!
//! A minimal but complete blockchain implementation in Rust starting with a
//! pre-mined genesis block. Educational foundation for mesh-native value transfer,
//! agent economies, and decentralized coordination.
//!
//! Run:
//!   cargo run -- show
//!   cargo run -- mine "My important data or agent reward"

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

/// Represents a single block in the Lynx-Coin blockchain.
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
    /// Create the hardcoded Genesis Block (Block #0).
    /// This is the immutable root of trust for Lynx-Coin.
    pub fn genesis() -> Self {
        // Pre-computed valid values for difficulty = 4
        // (hash starts with 4 leading zeros)
        let timestamp = 1781201253; // Unix timestamp used during genesis creation
        let data = serde_json::json!({
            "message": "Lynx-Coin Genesis Block",
            "allocations": {
                "bootstrap_fund": 8_000_000,
                "development_fund": 5_000_000,
                "community_ecosystem": 6_000_000,
                "strategic_partners": 2_000_000
            },
            "total_supply": 21_000_000,
            "note": "Immutable starting state for Lynx-Coin mesh-native economy. All future blocks are cryptographically linked to this root."
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

    /// Calculate the SHA-256 hash of the block header.
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

    /// Mine a new block (Proof of Work).
    /// Increments nonce until the hash meets the difficulty target.
    pub fn mine_block(data: String, prev_hash: String, difficulty: u32) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut nonce = 0u64;
        let target = "0".repeat(difficulty as usize);

        println!("Mining new block with difficulty {}...", difficulty);

        loop {
            let block = Self {
                index: 0, // temporary, will be set by chain
                timestamp,
                data: data.clone(),
                prev_hash: prev_hash.clone(),
                hash: String::new(),
                nonce,
                difficulty,
            };

            let hash = block.calculate_hash();

            if hash.starts_with(&target) {
                println!("Block mined! Nonce: {} | Hash: {}", nonce, hash);
                return Self {
                    index: 0,
                    timestamp,
                    data,
                    prev_hash,
                    hash,
                    nonce,
                    difficulty,
                };
            }

            nonce += 1;

            // Progress indicator for long mines
            if nonce % 100_000 == 0 {
                print!(".");
                use std::io::{self, Write};
                let _ = io::stdout().flush();
            }
        }
    }

    /// Human-readable timestamp
    pub fn datetime(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(self.timestamp, 0).unwrap_or_default()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Block #{} [{} ]", self.index, self.datetime())?;
        writeln!(f, "  Hash:      {}", self.hash)?;
        writeln!(f, "  Prev Hash: {}", self.prev_hash)?;
        writeln!(f, "  Nonce:     {} (difficulty: {})", self.nonce, self.difficulty)?;
        writeln!(f, "  Data:      {}", self.data)?;
        Ok(())
    }
}

/// The Lynx-Coin blockchain container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    /// Initialize a new chain with the genesis block.
    pub fn new() -> Self {
        let genesis = Block::genesis();
        Self {
            chain: vec![genesis],
        }
    }

    /// Get the latest block.
    pub fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    /// Add a mined block to the chain (after validation).
    pub fn add_block(&mut self, mut block: Block) {
        block.index = self.chain.len() as u64;
        // Recalculate hash with correct index (in case it was mined with placeholder)
        // For simplicity in v0.1 we re-mine if needed, but here we trust the miner
        self.chain.push(block);
    }

    /// Validate the entire chain integrity.
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Check hash linkage
            if current.prev_hash != previous.hash {
                eprintln!("Invalid prev_hash at block {}", i);
                return false;
            }

            // Check that the stored hash is correct
            if current.hash != current.calculate_hash() {
                eprintln!("Invalid hash at block {}", i);
                return false;
            }

            // Check difficulty
            let target = "0".repeat(current.difficulty as usize);
            if !current.hash.starts_with(&target) {
                eprintln!("Difficulty not met at block {}", i);
                return false;
            }
        }
        true
    }

    /// Mine and add a new block with the given data.
    pub fn mine_and_add(&mut self, data: String, difficulty: u32) {
        let prev_block = self.latest_block();
        let mut new_block = Block::mine_block(
            data,
            prev_block.hash.clone(),
            difficulty,
        );
        new_block.index = self.chain.len() as u64;
        // Ensure final hash is correct
        new_block.hash = new_block.calculate_hash();
        self.chain.push(new_block);
    }
}

/// CLI definition
#[derive(Parser)]
#[command(name = "lynx-coin")]
#[command(about = "Lynx-Coin blockchain CLI - Genesis block and simple PoW chain", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display the full blockchain
    Show,

    /// Mine a new block with custom data and append it
    Mine {
        /// Data to store in the new block (JSON string or plain text)
        data: String,

        /// Mining difficulty (leading zeros). Default: 4
        #[arg(short, long, default_value_t = 4)]
        difficulty: u32,
    },

    /// Validate the entire chain
    Validate,
}

fn main() {
    let cli = Cli::parse();
    let mut chain = Blockchain::new();

    match cli.command {
        Commands::Show => {
            println!("\n=== Lynx-Coin Blockchain ===\n");
            for block in &chain.chain {
                println!("{}", block);
                println!("────────────────────────────────────────");
            }
            println!("Chain length: {} blocks", chain.chain.len());
            println!("Chain valid: {}", chain.is_valid());
        }

        Commands::Mine { data, difficulty } => {
            println!("\nMining new block with data: {}", data);
            chain.mine_and_add(data, difficulty);

            println!("\n=== Updated Chain ===\n");
            for block in &chain.chain {
                println!("{}", block);
                println!("────────────────────────────────────────");
            }
            println!("New block added successfully!");
        }

        Commands::Validate => {
            let valid = chain.is_valid();
            println!("Chain validation result: {}", if valid { "VALID ✓" } else { "INVALID ✗" });
            if !valid {
                std::process::exit(1);
            }
        }
    }
}
