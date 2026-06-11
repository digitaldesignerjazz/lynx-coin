# Lynx-Coin

**Lynx-Coin (LYNX)** is a decentralized, mesh-native blockchain designed as the value and coordination layer for advanced distributed systems.

It begins with a carefully crafted **genesis block** and provides a clean, extensible Rust foundation for:

- Agent economies and autonomous swarms
- Mesh network incentives (NovaNet / QNET / xMesh integration)
- Hardware oracles and IoT value transfer
- Secure, auditable data anchoring
- Future smart contract and DeFi primitives

This project is the starting point for Lynx-Coin as part of a broader ecosystem that includes Nexus Core (runtime orchestration), Grok Launcher-style tooling, and privacy-focused networking.

## Vision

Lynx-Coin is built for the next generation of decentralized infrastructure:

- **Low-latency & high-throughput** where it matters (agent-to-agent micropayments, real-time mesh coordination)
- **Self-sovereign & privacy-preserving** by default
- **Interoperable** with existing mesh protocols and future L2s
- **Developer-friendly** Rust implementation with clear extension points

The genesis block establishes the initial token distribution and immutable starting state. All subsequent blocks are cryptographically linked and validated.

## Genesis Block Specifications

The genesis block (Block #0) was created with the following parameters:

- **Index**: 0
- **Timestamp**: 2026-06-11 (Unix timestamp embedded)
- **Data**: JSON object containing initial allocation and network bootstrap message
- **Previous Hash**: `0000000000000000000000000000000000000000000000000000000000000000`
- **Difficulty**: 4 (hash must start with 4 leading zeros in hex for demonstration; easily adjustable)
- **Nonce**: The value that satisfies the PoW for the genesis data

**Initial Allocation (embedded in genesis data)**:
- `bootstrap_fund`: 8_000_000 LYNX (network bootstrap, liquidity, incentives)
- `development_fund`: 5_000_000 LYNX (core development, security audits, tooling)
- `community_ecosystem`: 6_000_000 LYNX (airdrops, grants, agent swarm rewards)
- `strategic_partners`: 2_000_000 LYNX (early mesh node operators, hardware partners)

**Total Genesis Supply**: 21_000_000 LYNX (hard cap for initial phase; future emissions decided by governance)

The genesis block is **pre-mined** and hardcoded. Its hash serves as the root of trust for the entire chain.

## Quick Start

### Prerequisites

- Rust 1.75+ (recommended latest stable)
- `cargo`

### Build & Run

```bash
git clone https://github.com/digitaldesignerjazz/lynx-coin.git
cd lynx-coin
cargo build --release

# Show the genesis block and mine a few example blocks
./target/release/lynx-coin

# Mine a new block with custom data
./target/release/lynx-coin mine "Agent swarm reward distribution - Q3 2026"

# View the full chain
./target/release/lynx-coin show
```

## Architecture

```
Block
  ├── index: u64
  ├── timestamp: i64
  ├── data: String (or structured JSON)
  ├── prev_hash: String (64 hex chars)
  ├── hash: String
  ├── nonce: u64
  └── difficulty: u32

Blockchain
  └── vec of validated Blocks
  └── validate_chain()
  └── add_block()
  └── mine_block(data)
```

**Core Components** (in `src/main.rs` for v0.1 simplicity):

- `Block` struct with SHA-256 hashing
- Proof-of-Work mining loop (simple but educational; production would use more sophisticated consensus)
- `Blockchain` container with validation
- CLI via `clap` for `show`, `mine`, and future commands

Future modules (separate crates or features):
- Transaction pool & mempool
- P2P networking (libp2p or direct Yggdrasil integration)
- Account model or UTXO
- Smart contracts (or Move / WASM)
- Light client & SPV proofs
- Integration with **Nexus Core** runtime for agent orchestration and mesh supervision

## Design Decisions & Trade-offs

| Aspect              | Choice                          | Rationale / Trade-off                          |
|---------------------|---------------------------------|------------------------------------------------|
| Hashing             | SHA-256                         | Battle-tested, widely supported. Future: BLAKE3 or Poseidon for ZK |
| Consensus (v0.1)    | Simple PoW                      | Educational clarity. Easy to understand & modify. Later: PoS, PoA, or hybrid |
| Data field          | String / JSON                   | Flexible for arbitrary payloads (agent messages, oracle data, etc.) |
| Difficulty          | Adjustable leading zeros        | Easy to tune for demo vs production            |
| No real transactions yet | Simple data string         | Focus on chain integrity first. Transactions come next |

**Why Rust?** Memory safety + fearless concurrency = ideal for a core blockchain node that may later run alongside Nexus Core agents and mesh networking stacks.

## Roadmap

- [x] Genesis block + basic PoW chain
- [ ] Proper Transaction struct + signature verification (ed25519)
- [ ] Mempool and block production
- [ ] P2P layer (start with Yggdrasil or QUIC)
- [ ] Wallet CLI / key management
- [ ] Integration hooks for Nexus Core (event bus, agent spawning on chain events)
- [ ] Explorer / dashboard (egui or web)
- [ ] Testnet launch with real mesh node incentives
- [ ] Governance module (on-chain proposals)
- [ ] ZK or privacy features (future)

## Contributing

This is the genesis of Lynx-Coin. Pull requests that improve the chain logic, add transactions, or integrate with mesh/AI components are highly encouraged.

Open issues for:
- Better difficulty adjustment algorithm
- Persistent storage (sled/redb)
- Network protocol

## License

MIT License. See LICENSE file.

---

*"The lynx moves silently through the mesh — fast, aware, and sovereign."*

**Built as part of the NovaNet / QNET vision.**