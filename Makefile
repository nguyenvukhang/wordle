dev:
	cargo build --release
	RUST_LOG=info ./target/release/wordle

test:
	cargo test
