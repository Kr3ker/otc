# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Solana smart contract project using Anchor with Arcium integration for confidential computing. The project enables encrypted computations on-chain using Arcium as a co-processor.

## Commands

### Build and Test
```bash
arcium build                    # Build Solana programs
arcium test                     # Run tests (starts localnet + Arcium)
yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/**/*.ts"  # Run tests directly
```

### Frontend
```bash
yarn dev                        # Start Next.js dev server (runs frontend workspace)
yarn workspace frontend build   # Build frontend
yarn workspace frontend lint    # Lint frontend
```

### Code Quality
```bash
yarn lint                       # Check formatting with Prettier
yarn lint:fix                   # Fix formatting with Prettier
```

## Architecture

### Two-Location Development Pattern

Code is written in three places:

1. **`programs/otc/`** - Standard Anchor program (Rust)
   - Handles plaintext Solana operations
   - Defines instructions that queue confidential computations
   - Receives callbacks with encrypted results

2. **`encrypted-ixs/`** - Arcis encrypted instructions (Rust)
   - Defines confidential computing operations using the Arcis framework
   - Operations execute off-chain on the Arcium network
   - Uses `#[encrypted]` and `#[instruction]` macros

3. **`frontend/`** - Next.js web application
   - Next.js 16.1.1 with React 19
   - TypeScript 5
   - Tailwind CSS v4 for styling
   - Configured as a yarn workspace

### Confidential Computation Flow

Each confidential operation requires three components:

1. **Initialization instruction** (`init_*_comp_def`) - Registers the computation definition
2. **Queue instruction** - Sends encrypted inputs to Arcium via `queue_computation()`
3. **Callback instruction** (`*_callback`) - Receives and verifies encrypted outputs with `#[arcium_callback]`

### Key Macros

- `#[arcium_program]` - Marks the main program module
- `#[queue_computation_accounts("name", payer)]` - Generates required accounts for queuing
- `#[callback_accounts("name")]` - Generates required accounts for callbacks
- `#[init_computation_definition_accounts("name", payer)]` - Generates init accounts
- `#[arcium_callback(encrypted_ix = "name")]` - Links callback to encrypted instruction

### Encryption Pattern

Uses x25519 key exchange with RescueCipher:
```typescript
const sharedSecret = x25519.getSharedSecret(privateKey, mxePublicKey);
const cipher = new RescueCipher(sharedSecret);
const ciphertext = cipher.encrypt(plaintext, nonce);
```

## Key Dependencies

- **Anchor** 0.32.1 - Solana framework
- **Arcium** 0.5.4 - `arcium-anchor`, `arcium-client`, `arcium-macros`
- **Arcis** 0.5.4 - Encrypted instruction framework
- **Next.js** 16.1.1 - Frontend (in `frontend/` workspace)

## Testing

Tests require Arcium localnet running (configured in `Arcium.toml`):
- 2 nodes
- 60 second timeout
- Cerberus MPC backend

Tests expect a Solana keypair at `~/.config/solana/id.json`.