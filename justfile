# Install some necessary tools
install:
    cargo install cargo-watch

# Check formatting
dev:
    cargo watch -w src -w data -w templates -x 'run' -s "open -g docs/index.html"

# Check formatting
format:
    cargo fmt --all
    cargo clippy -- -D warnings

# Run tests   
test:
    cargo t --all
    
# Utility to run everything related to the project:   
all: install format test
