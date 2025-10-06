# Triangle Agency Bot

A Telegram bot for rolling dice in the [Triangle Agency TTRPG](https://shop.hauntedtable.games/) system.

## What it does

This bot handles the Triangle Agency dice mechanic: roll 6d4 and count how many 3's you get. 

The bot shows your roll as triangles (▲ for a 3, ▽ for anything else) and calculates 🌀 **chaos** based on how many dice failed.

## Commands

- `/roll` or `/r` - Roll 6d4
- `/help` or `/h` - Show available commands

## Setup

You'll need Rust nightly (currently using nightly-2025-09-26) because this uses some Edition 2024 features.

```bash
cargo build
cargo run
```

The bot needs a `TELOXIDE_TOKEN` environment variable with your Telegram bot token. 
## Deployment

Built to run on [Shuttle](https://shuttle.dev). The bot token is managed through Shuttle's secrets:

```bash
shuttle deploy
```

Make sure you've set `TELOXIDE_TOKEN` in your Shuttle secrets first.

## How the dice work

- Roll 6d4, count the 3's
- **0 threes**: Failure ❌ (chaos = 6)
- **1-2 or 4-6 threes**: Success ✅ (chaos = number of non-3's)
- **Exactly 3 threes**: Triscendence ✨ (chaos = 0)

Example output:
```
✅  ▲ ▽ ▲ ▽ ▽ ▽
🌀 4
```

## Tech stack

- Rust (Edition 2024)
- Teloxide for the Telegram bot framework
- Shuttle for deployment
- Tokio for async runtime

## License

Public domain. See [UNLICENSE](UNLICENSE) for details.
