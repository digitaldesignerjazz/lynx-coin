//! Lynx-Coin CLI (thin binary)
//!
//! Uses the lynx_coin library for all core logic.

use clap::{Parser, Subcommand};
use lynx_coin::{Blockchain, Transaction, Wallet};
use std::fs;

#[derive(Parser)]
#[command(name = "lynx-coin")]
#[command(about = "Lynx-Coin v0.3 - Library + CLI with transactions in blocks + balances")]
#[command(version = "0.3.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Show,
    Mine {
        #[arg(short, long)]
        data: Option<String>,
        #[arg(short, long, default_value_t = 4)]
        difficulty: u32,
    },
    Validate,

    Keygen,
    CreateTx {
        from: String,
        to: String,
        amount: u64,
        #[arg(long)]
        data: Option<String>,
    },
    SignTx {
        tx: String,
        private_key: String,
    },
    VerifyTx {
        tx: String,
    },

    /// Show balance of an address
    Balance {
        address: String,
    },

    /// Create and sign a tx directly from wallet
    Send {
        from: String,
        to: String,
        amount: u64,
        #[arg(long)]
        data: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut chain = Blockchain::new();
    let mut wallet = Wallet::new();

    match cli.command {
        Commands::Show => {
            println!("\n=== Lynx-Coin Blockchain v0.3 ===\n");
            for block in &chain.chain {
                println!("{}", block);
            }
            println!("Chain valid: {} | Blocks: {}", chain.is_valid(), chain.chain.len());
        }
        Commands::Mine { data, difficulty } => {
            // For demo: create a simple transaction if data provided
            let txs = if let Some(d) = data {
                vec![Transaction::new("demo".to_string(), "demo_recipient".to_string(), 1, Some(d))]
            } else {
                vec![]
            };
            match chain.mine_and_add(txs, difficulty) {
                Ok(()) => println!("Block mined and added successfully!"),
                Err(e) => eprintln!("Failed to add block: {}", e),
            }
        }
        Commands::Validate => {
            println!("Chain valid: {}", chain.is_valid());
        }

        Commands::Keygen => {
            let addr = wallet.generate_key();
            println!("New address generated: {}", addr);
        }
        Commands::CreateTx { from, to, amount, data } => {
            let tx = Transaction::new(from, to, amount, data);
            println!("{}", serde_json::to_string_pretty(&tx).unwrap());
        }
        Commands::SignTx { tx, private_key } => {
            let mut tx: Transaction = if tx.starts_with('{') {
                serde_json::from_str(&tx).unwrap()
            } else {
                serde_json::from_str(&fs::read_to_string(tx).unwrap()).unwrap()
            };
            if tx.sign(&private_key).is_ok() {
                println!("Signed:\n{}", serde_json::to_string_pretty(&tx).unwrap());
            }
        }
        Commands::VerifyTx { tx } => {
            let tx: Transaction = if tx.starts_with('{') {
                serde_json::from_str(&tx).unwrap()
            } else {
                serde_json::from_str(&fs::read_to_string(tx).unwrap()).unwrap()
            };
            match tx.verify() {
                Ok(true) => println!("✓ VALID"),
                Ok(false) => println!("✗ INVALID"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Commands::Balance { address } => {
            println!("Balance of {}: {} LYNX", address, chain.get_balance(&address));
        }

        Commands::Send { from, to, amount, data } => {
            match wallet.create_signed_transaction(&from, to, amount, data) {
                Ok(tx) => {
                    println!("Created & signed transaction:\n{}", serde_json::to_string_pretty(&tx).unwrap());
                    // In real usage you would broadcast this tx
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
