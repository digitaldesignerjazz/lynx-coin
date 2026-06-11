//! Blockchain and simple account-based ledger with balance tracking.

use crate::block::Block;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    /// Simple account model balances (address -> balance)
    pub balances: HashMap<String, u64>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::genesis();
        let mut balances = HashMap::new();

        // Initialize balances from genesis transactions
        for tx in &genesis.transactions {
            *balances.entry(tx.to.clone()).or_insert(0) += tx.amount;
        }

        Self {
            chain: vec![genesis],
            balances,
        }
    }

    pub fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    /// Add a mined block after validating signatures and updating balances.
    pub fn add_block(&mut self, mut block: Block) -> Result<(), String> {
        if !block.validate_signatures() {
            return Err("One or more transaction signatures are invalid".to_string());
        }

        // Apply transactions to balances (very simple account model)
        for tx in &block.transactions {
            if tx.from != "GENESIS" {
                let from_balance = self.balances.get(&tx.from).copied().unwrap_or(0);
                if from_balance < tx.amount {
                    return Err(format!("Insufficient balance for {}", tx.from));
                }
                *self.balances.entry(tx.from.clone()).or_insert(0) -= tx.amount;
            }
            *self.balances.entry(tx.to.clone()).or_insert(0) += tx.amount;
        }

        block.index = self.chain.len() as u64;
        block.hash = block.calculate_hash();
        self.chain.push(block);
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.prev_hash != previous.hash {
                return false;
            }
            if current.hash != current.calculate_hash() {
                return false;
            }
            if !current.validate_signatures() {
                return false;
            }
        }
        true
    }

    /// Mine a new block and add it (validates signatures + updates balances)
    pub fn mine_and_add(&mut self, transactions: Vec<Transaction>, difficulty: u32) -> Result<(), String> {
        let prev_block = self.latest_block();
        let new_block = Block::mine_block(transactions, prev_block.hash.clone(), difficulty);
        self.add_block(new_block)
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }
}