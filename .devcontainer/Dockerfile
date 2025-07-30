FROM mcr.microsoft.com/devcontainers/rust:1

# Set environment variable for Deno install path
ENV DENO_INSTALL="/root/.deno"

# Install system packages, tools, and cleanup after install
RUN sudo apt-get update \
    && sudo apt-get install -y \
        bash \
        curl \
        python3 \
        python3-pip \
        python3-venv \
        unzip \
    \
    # Install just
    && curl -fsSL https://just.systems/install.sh | bash -s -- --to /usr/local/bin \
    \
    # Install Deno
    && curl -fsSL https://deno.land/install.sh | bash \
    && ln -s $DENO_INSTALL/bin/deno /usr/local/bin/deno \
    \
    # Install ngrok CLI - connect from outside instead, through ngrok http 3000
    # && curl -sSL https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-stable-linux-amd64.zip -o /tmp/ngrok.zip \
    # && unzip /tmp/ngrok.zip -d /usr/local/bin \
    # && chmod +x /usr/local/bin/ngrok \
    # && rm /tmp/ngrok.zip \
    # \
    # Create symlink: python -> python3
    && sudo ln -s /usr/bin/python3 /usr/bin/python || true \
    \
    # Clean APT cache and temp files
    && sudo apt-get clean \
    && sudo rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Copy Python dependencies and install with no pip cache
COPY services/patient-summary-agent/requirements.txt /tmp/requirements.txt
# Create a virtual environment and install Python packages there
RUN python3 -m venv /opt/venv \
    && /opt/venv/bin/pip install --upgrade pip \
    && /opt/venv/bin/pip install --no-cache-dir -r /tmp/requirements.txt \
    && rm /tmp/requirements.txt

ENV PATH="/opt/venv/bin:$PATH"

# Verify installations
# RUN just --version \
#  && deno --version \
#  && ngrok version \
#  && python --version \
#  && pip --version

# Set working directory
WORKDIR /workspace

# Copy source code
COPY . .

# Optionally build workspace using just
RUN just build
