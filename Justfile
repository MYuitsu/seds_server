# Tải dependencies cho workspace
fetch:
    cargo fetch

# Build workspace (tự động fetch trước)
build: fetch
    : cargo build

# Chạy project chính
dev:
    just frontend & just gateway

# Test toàn bộ workspace
test:
    cargo test

# Format code
fmt:
    cargo fmt

gateway:
    # nạp env và chạy service
    cd services/api-gateway && \
    cargo run

# Lint code
lint:
    cargo clippy --all-targets --all-features -- -D warnings
frontend:
    cd frontend && ng serve

# Chạy riêng cả frontend + backend của patient-summary
patient-summary-dev:
    just patient-summary-backend &
    just patient-summary-frontend &
    just patient-summary-agent &
    wait

# Backend patient-summary
patient-summary-backend:
    cd services/patient-summary-service && cargo run

# Frontend patient-summary
patient-summary-frontend:
    cd frontend/patient-summary-frontend && \
    deno task start

# Agent patient-summary
patient-summary-agent:
    cd services/patient-summary-agent && \
    uvicorn app.main:app --host 0.0.0.0 --port 3020 --reload
