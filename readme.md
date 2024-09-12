# EchoID

EchoID is a decentralized identity and reputation system built on the Solana blockchain. It allows users to create unique aliases linked to multiple blockchain addresses and manage their cross-chain reputation.

## Features

- Alias Registration: Users can register unique aliases in the format `username@projectsuffix`.
- Cross-Chain Address Mapping: Each alias can be linked to multiple blockchain addresses (currently supporting Solana and EVM-compatible chains).
- Reputation System: Each alias has an associated reputation score that can be updated by authorized entities.
- Admin Controls: Certain operations, like updating reputation, are restricted to admin accounts for security.

## Prerequisites

- Rust and Cargo (latest stable version)
- Solana CLI tools (v1.16.0 or later)
- Node.js (v14 or later) and npm
- Anchor Framework (v0.28.0 or later)

## Setup

1. Clone the repository:

   ```
   git clone https://github.com/starc007/echo_id_contracts.git
   cd echo_id_contract
   ```

2. Install dependencies:

   ```
   npm install
   ```

3. Build the program:
   ```
   anchor build
   ```

## Testing

To run the test suite:

```
anchor test
```

## Project Structure

- `programs/echo_id_contract/src/`: Contains the Rust source code for the Solana program.
  - `lib.rs`: Main entry point for the Solana program.
  - `instructions/`: Contains individual instruction handlers.
  - `state.rs`: Defines the account structures used in the program.
  - `error.rs`: contains the errors used in the program.
- `tests/`: Contains the TypeScript test files.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This project is in development and not audited. Use at your own risk.
