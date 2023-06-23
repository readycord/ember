# Show all available commands
help:
	@just --list

# Compile and run the program in dev mode
dev:
	cargo run

# Run clippy to analyze your code
check:
	cargo clippy --workspace --all-features -- -D warnings

# Format your code
fmt:
	cargo +nightly fmt --all

# Run tests
test:
	cargo test --workspace --all-features

# Generate documentation
doc:
	cargo doc --all-features

# Generate git hooks
hooks:
	./hooks.sh

