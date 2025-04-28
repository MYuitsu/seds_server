# Tải dependencies cho workspace
fetch:
    cargo fetch

# Build workspace (tự động fetch trước)
build: fetch
    : cargo build

# Chạy project chính
dev:
    cargo run

# Test toàn bộ workspace
test:
    cargo test

# Format code
fmt:
    cargo fmt

# Lint code
lint:
    cargo clippy --all-targets --all-features -- -D warnings