# 🏆 On-Chain Leaderboard — Soroban Smart Contract

A seasonal leaderboard built on Stellar's Soroban platform.
Players submit scores from their wallets; only their personal best is kept.
An admin can reset the board and start a new season at any time.

## Description

This smart contract implements a fully on-chain leaderboard system on the Stellar testnet.
Any wallet can submit a score with a username. The contract only keeps each player's highest
score. An admin wallet has exclusive rights to wipe the board and advance the season counter,
enabling competitive seasonal resets.

## Features

- Submit scores tied to a wallet address
- Only the personal best score per wallet is stored
- Fetch the full leaderboard or just the top N players (sorted)
- Season management — admin can reset all scores and increment the season
- Fully on-chain, no off-chain database needed

## Testnet Smart Contract ID

```
CBQBPR35VS3PTRGPMU2LFTZNXWKNCO66AE2UJZODHJKFTA3FYLKSZR2Y
```

Network: **Stellar Testnet** (Test SDF Network ; September 2015)

## Functions

| Function | Who can call | Description |
|---|---|---|
| `initialize(admin)` | Anyone (once) | Register the admin wallet and start Season 1 |
| `submit_score(player, username, score)` | Any wallet | Submit a score; only updates if it beats the current best |
| `get_leaderboard()` | Anyone | Return all players (unsorted) |
| `get_top(n)` | Anyone | Return the top N players sorted by score descending |
| `get_season()` | Anyone | Return the current season number |
| `reset_season(admin)` | Admin only | Wipe all scores and advance the season counter |

## Project Structure

```
contracts/
└── leaderboard/
    └── src/
        ├── lib.rs      ← main contract logic
        └── test.rs     ← unit tests
    ├── Cargo.toml
```

## How to Invoke via CLI

```bash
# Initialize (run once after deploy)
stellar contract invoke --id CBQBPR35VS3PTRGPMU2LFTZNXWKNCO66AE2UJZODHJKFTA3FYLKSZR2Y --source-account world --network testnet -- initialize --admin <YOUR_G_ADDRESS>

# Submit a score
stellar contract invoke --id CBQBPR35VS3PTRGPMU2LFTZNXWKNCO66AE2UJZODHJKFTA3FYLKSZR2Y --source-account world --network testnet -- submit_score --player <YOUR_G_ADDRESS> --username "Alice" --score 1000

# Get top 3
stellar contract invoke --id CBQBPR35VS3PTRGPMU2LFTZNXWKNCO66AE2UJZODHJKFTA3FYLKSZR2Y --source-account world --network testnet -- get_top --n 3

# Reset season (admin only)
stellar contract invoke --id CBQBPR35VS3PTRGPMU2LFTZNXWKNCO66AE2UJZODHJKFTA3FYLKSZR2Y --source-account world --network testnet -- reset_season --admin <YOUR_G_ADDRESS>
```

## Built With

- [Soroban SDK](https://soroban.stellar.org) — Stellar smart contract platform
- [Soroban Studio](https://soroban.studio) — Browser-based IDE
- AI assistance via Claude (Anthropic)