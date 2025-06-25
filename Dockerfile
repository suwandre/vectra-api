# Multi-stage build for optimized Axum application
# Builder stage - compiles the Rust application
FROM rust:slim-bullseye as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy actual source code
COPY crates ./crates
COPY src ./src

# Build the actual application
RUN cargo build --release --bin app

# Runtime stage - minimal image for deployment
FROM debian:bullseye-slim as runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -ms /bin/bash vectrauser
USER vectrauser

WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /app/target/release/app /app/vectra-dex

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080

# Expose the port that Elastic Beanstalk expects
EXPOSE 8080

# Health check endpoint for Elastic Beanstalk
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Start the application
ENTRYPOINT ["/app/vectra-dex"]
