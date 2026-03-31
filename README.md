# StellarPass Contracts

Soroban smart contracts for the StellarPass event ticketing dApp, built with Rust on the Stellar blockchain.

## Contracts

| Contract | Description |
|---|---|
| `event_contract` | Event creation, management, and cancellation |
| `ticket_contract` | Soulbound (non-transferable) NFT ticket minting and verification |
| `payment_contract` | XLM and USDC payment processing and refunds |

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli)

```bash
# Install Soroban CLI
cargo install --locked stellar-cli --features opt
```

## Project Structure

```
stellarpass-contracts/
├── contracts/
│   ├── event_contract/
│   │   ├── src/
│   │   │   └── lib.rs          # Event contract logic
│   │   └── Cargo.toml
│   ├── ticket_contract/
│   │   ├── src/
│   │   │   └── lib.rs          # Ticket minting & verification logic
│   │   └── Cargo.toml
│   └── payment_contract/
│       ├── src/
│       │   └── lib.rs          # Payment & refund logic
│       └── Cargo.toml
├── docs/
│   └── contract-specs.md       # Detailed contract function specs
├── Cargo.toml                  # Workspace Cargo config
├── .github/
│   └── workflows/
│       └── ci.yml              # Build & test CI pipeline
└── README.md
```

## Getting Started

```bash
# Clone the repo
git clone https://github.com/your-org/stellarpass-contracts.git
cd stellarpass-contracts

# Build all contracts
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test
```

## Deploying to Testnet

```bash
# Configure testnet identity
stellar keys generate --global alice --network testnet

# Deploy event contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/event_contract.wasm \
  --source alice \
  --network testnet

# Deploy ticket contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ticket_contract.wasm \
  --source alice \
  --network testnet

# Deploy payment contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/payment_contract.wasm \
  --source alice \
  --network testnet
```

## Contract Function Reference

### Event Contract
| Function | Description |
|---|---|
| `create_event` | Create a new event on-chain |
| `get_event` | Fetch event data by ID |
| `update_event` | Update event details (organizer only) |
| `cancel_event` | Cancel event and trigger refunds |

### Ticket Contract
| Function | Description |
|---|---|
| `mint_ticket` | Mint a soulbound ticket to an attendee wallet |
| `verify_ticket` | Verify ticket ownership |
| `check_in` | Mark ticket as checked-in at entry |
| `revoke_ticket` | Revoke a ticket (organizer only) |

### Payment Contract
| Function | Description |
|---|---|
| `process_payment` | Handle XLM or USDC ticket payment |
| `refund` | Refund attendee on event cancellation |

## Environment Variables

Create a `.env` file at the root:

```env
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
EVENT_CONTRACT_ID=<deployed_event_contract_id>
TICKET_CONTRACT_ID=<deployed_ticket_contract_id>
PAYMENT_CONTRACT_ID=<deployed_payment_contract_id>
```

## Contributing

See [CONTRIBUTING.md](./docs/CONTRIBUTING.md) for guidelines.

## License

MIT
