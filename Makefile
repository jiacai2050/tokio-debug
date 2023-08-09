
serve:
	go run server/main.go

run:
	RUST_LOG=info cargo run --release

run-trace:
	RUST_LOG=trace cargo run --release
