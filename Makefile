build:
	cargo build --release
run:
	./target/release/qa-api-rs
dev:
	systemfd --no-pid -s http::3000 -- cargo watch -x run
