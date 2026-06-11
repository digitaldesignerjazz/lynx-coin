# Lynx-Coin v0.3

**Major milestone**: Lynx-Coin is now a proper Rust **library** (`lynx_coin`) with:

- Transactions fully integrated into Blocks
- Signature validation on every block addition
- Simple account-based balance tracking
- Wallet / keystore module
- Clean library structure ready to be used as a dependency by **Nexus Core**

## New Architecture

```
src/
├── lib.rs          # Public API + module exports
├── block.rs        # Block with Vec<Transaction> + signature validation
├── chain.rs        # Blockchain + account balances
├── transaction.rs  # Ed25519 signed transactions
├── wallet.rs       # In-memory keystore + signed tx helper
└── main.rs         # Thin CLI
```

## Key Improvements in v0.3

1. **Transactions inside Blocks** — `Block` now contains `Vec<Transaction>` instead of plain data.
2. **Signature validation** — `block.validate_signatures()` + enforced in `chain.add_block()`.
3. **Balance tracking** — Simple account model (`balances: HashMap<String, u64>`).
4. **Wallet module** — Easy key generation and `create_signed_transaction()`.
5. **Library ready** — `lynx-coin` can now be added as a dependency in Nexus Core or other projects.

## Usage Examples

```bash
cargo run -- keygen
cargo run -- balance <address>
cargo run -- send <from> <to> 1000000 --data "Agent reward"
cargo run -- show
```

## P2P & Mesh Integration (Planned / Skeleton)

A `network` module skeleton is prepared for future integration with:
- Yggdrasil / custom xMesh
- libp2p or QUIC
- Nexus Core's networking layer

Transaction propagation and block gossip will be added in the next phase.

## Next Steps

- Full UTXO model option
- Encrypted keystore
- Real P2P networking
- Smart contract / script support
- Tight integration with Nexus Core EventBus

The foundation for a production-grade mesh-native cryptocurrency is now in place.