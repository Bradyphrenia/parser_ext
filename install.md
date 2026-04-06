cargo clean
cargo build --bins --features pg17 --no-default-features
cargo pgrx install --pg-config /opt/homebrew/opt/postgresql@17/bin/pg_config