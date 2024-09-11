build:
	cargo build --release
run:
	./target/release/qa-api-rs
dev:
	fd rs | entr cargo run
