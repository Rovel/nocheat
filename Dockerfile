FROM rust:latest

# Install dependencies for cross-compilation
RUN apt-get update && apt-get install -y \
    gcc-multilib \
    g++-multilib \
    gcc-mingw-w64 \
    g++-mingw-w64 \
    musl-tools \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Install macOS cross-compilation tools
RUN rustup target add x86_64-apple-darwin aarch64-apple-darwin
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add x86_64-unknown-linux-gnu x86_64-unknown-linux-musl

# Install the cross tool
RUN cargo install cross

WORKDIR /app

# Copy the project files
COPY . .

# Command to build for all targets
CMD ["./build-all.sh"]