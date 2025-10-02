build:
	solana-verify build

program_address:
	@solana address -k ../../target/deploy/solana-keypair.json

fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-targets --all-features -- -D warnings

test:
	cargo test-sbf -- --nocapture

start_validator:
	solana-test-validator

deploy: build
	solana program deploy ../../target/deploy/solana.so
