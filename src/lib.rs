//! Lynx-Coin Core Library
//!
//! This crate provides the core blockchain primitives for Lynx-Coin:
//! - Block and Blockchain with embedded signed Transactions
//! - Ed25519 transaction signing and verification
//! - Basic account-based balance tracking
//! - Wallet / keystore abstractions (in-memory + file)
//!
//! This library is designed to be depended on by Nexus Core and other components.

pub mod block;
pub mod transaction;
pub mod wallet;
pub mod chain;

// Re-exports for convenience
pub use block::Block;
pub use transaction::Transaction;
pub use wallet::Wallet;
pub use chain::Blockchain;

/// Current version of the Lynx-Coin protocol / library
pub const VERSION: &str = "0.3.0";