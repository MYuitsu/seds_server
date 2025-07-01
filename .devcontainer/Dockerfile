
FROM mcr.microsoft.com/devcontainers/rust:1

# Cài thêm các tool nếu cần, ví dụ just, cargo-watch
RUN sudo apt-get update \
 && sudo apt-get install -y curl \
 && curl -fsSL https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# Install Deno
ENV DENO_INSTALL="/root/.deno"
RUN curl -fsSL https://deno.land/install.sh | bash && \
    ln -s $DENO_INSTALL/bin/deno /usr/local/bin/deno

# Install ngrok CLI (binary)
RUN sudo apt-get install -y curl unzip bash && \
    curl -sSL https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-stable-linux-amd64.zip -o /tmp/ngrok.zip && \
    unzip /tmp/ngrok.zip -d /usr/local/bin && \
    chmod +x /usr/local/bin/ngrok && \
    rm /tmp/ngrok.zip

# Verify everything installed
RUN just --version && \
    deno --version && \
    ngrok version

WORKDIR /workspace

COPY . .

# Build workspace (nếu muốn)
RUN just build