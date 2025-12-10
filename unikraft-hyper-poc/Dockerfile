FROM rust:latest

WORKDIR /src

# Copy source files
COPY Cargo.toml .
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Copy the binary to the filesystem
RUN mkdir -p /target/fs0 && cp target/release/tokio-hyper-server /target/fs0/rust-server