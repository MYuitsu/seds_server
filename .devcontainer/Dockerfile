
FROM mcr.microsoft.com/devcontainers/rust:1

# Cài thêm các tool nếu cần, ví dụ just, cargo-watch
RUN sudo apt-get update \
 && sudo apt-get install -y curl \
 && curl -fsSL https://just.systems/install.sh | bash -s -- --to /usr/local/bin

WORKDIR /workspace

COPY . .

# Build workspace (nếu muốn)
RUN just build