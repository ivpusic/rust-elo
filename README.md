# rust-elo

[![Build Status](https://travis-ci.org/ivpusic/rust-elo.svg?branch=master)](https://travis-ci.org/ivpusic/rust-elo)

Rust crate for calculating player rating based on elo ranking system

## Installation

Add to your `Cargo.toml`

```bash
[dependencies]
elo = "0.2.0"
```

## Example usage

```rust
extern crate elo;

use elo::{EloRank, MatchWinner};

fn main() {
    let elo = EloRank { k: 32 };
    let player_a = 1200.0;
    let player_b = 1400.0;
    let winner = MatchWinner::PlayerA;

    let (_player_a_new_ranking, _player_b_new_ranking) = elo.calculate(player_a, player_b, winner);
}
```
