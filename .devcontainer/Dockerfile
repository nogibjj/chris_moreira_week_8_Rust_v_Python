# Base image for Rust
FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Update and install dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld python3 python3-pip \
    && apt-get autoremove -y && apt-get clean -y

# Set up Python environment for testing
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt

# Copy all files
COPY . .

# Build Rust project
WORKDIR /app/sqlite
RUN cargo build --release

# Default command
CMD ["cargo", "run"]
