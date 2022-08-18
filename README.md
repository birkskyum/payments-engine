# Payments Engine

Simple payments engine with the following functions:
- Deposits
- Withdrawals
- Disputes
- Resolves
- Chargebacks

Instructions for CLI
```rust
cargo run -- --help
```

Test with
```rust
cargo test
```

Execute a transactions file, and save accounts to csv. One of the test fixtures with sample transactions can be used to demonstrate this:
```rust
cargo run -- fixtures/chargeback_transactions.csv > accounts.csv
```



