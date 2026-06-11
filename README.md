# Lynx-Coin

**Lynx-Coin (LYNX) v0.2** — Decentralized blockchain with **ed25519 transaction signatures**.

This release adds proper cryptographic transaction authorization using Ed25519.

## What's New in v0.2

- Full `Transaction` struct with `from`, `to`, `amount`, `timestamp`, `data`
- Ed25519 keypair generation (`lynx-coin keygen`)
- Transaction creation, signing, and verification via CLI
- All signatures use canonical message serialization to prevent replay and malleability attacks

## Quick Start (v0.2)

```bash
git clone https://github.com/digitaldesignerjazz/lynx-coin.git
cd lynx-coin
cargo run -- keygen                    # Generate Ed25519 keypair
cargo run -- create-tx <FROM> <TO> 1000000 'Agent payment'
cargo run -- sign-tx '<json>' <PRIVATE_KEY>
cargo run -- verify-tx '<signed-json>'

cargo run -- show                      # View genesis + chain
cargo run -- mine "Block with future tx support"
```

## Transaction Signing Flow

1. `keygen` → Get private + public key
2. `create-tx` → Build unsigned transaction (JSON)
3. `sign-tx` → Attach Ed25519 signature using private key
4. `verify-tx` → Cryptographically verify the signature

The signature covers: `from + to + amount + timestamp + data`.

This is the foundation for secure value transfer in the Lynx-Coin network.

## Genesis Block (unchanged)

The genesis block remains the same as v0.1 (pre-mined with difficulty 4).

## Architecture (v0.2)

- `src/transaction.rs` — Ed25519 signing & verification logic
- `src/main.rs` — Blockchain + CLI (Block, Blockchain, Transaction integration)

Future: Transactions will be included inside blocks instead of plain `data` string.

## Security Notes

- Ed25519 provides strong security with small, fast signatures.
- Timestamp in every transaction helps mitigate replay attacks.
- Always keep private keys secret.
- This is still an educational implementation. Do not use with real value yet.

## Roadmap

- [x] Genesis block
- [x] Basic PoW
- [x] ed25519 Transaction signatures (v0.2)
- [ ] Include signed transactions inside blocks
- [ ] UTXO or account model
- [ ] P2P networking (Yggdrasil / libp2p)
- [ ] Integration with Nexus Core runtime

See full details in the original README content and source code.

*"The lynx signs its moves."*