build:
	cargo build --release
run:
	./target/release/rust-playground
dev:
	fd rs | entr cargo run
