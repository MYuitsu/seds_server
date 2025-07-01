
FROM mcr.microsoft.com/devcontainers/rust:1

# Cài thêm các tool nếu cần, ví dụ just, cargo-watch
RUN sudo apt-get update \
 && sudo apt-get install -y curl \
 && curl -fsSL https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# Install Deno
RUN curl -fsSL https://deno.land/install.sh | bash \
 && export DENO_INSTALL="/root/.deno" \
 && export PATH="$DENO_INSTALL/bin:$PATH"

WORKDIR /workspace

COPY . .

# Build workspace (nếu muốn)
RUN just build