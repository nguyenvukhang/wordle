main:
	make dev

test:
	cargo test

dev:
	cargo build --release
	./target/release/wordle
