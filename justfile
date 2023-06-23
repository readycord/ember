# Show all available commands
help:
	@just --list

# Build the project
build:
	cargo build --release --all-features
	wasm-pack build --release --out-name ember --out-dir ./js/pkgs
	node ./js/fix-npm.js

# Run clippy to analyze your code
check:
	cargo clippy --workspace --all-features -- -D warnings

# Format your code
fmt:
	cargo +nightly fmt --all

# Run tests
test:
	cargo test --workspace --all-features -- --nocapture

# Generate documentation
doc:
	cargo doc --all-features

# Generate git hooks
hooks:
	./hooks.sh

