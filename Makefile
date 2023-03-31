debug:
	cargo test outcome_test
	# cargo test outcome_str_test

main:
	make dev

test:
	cargo test

dev:
	cargo build --release
	./target/release/wordle
