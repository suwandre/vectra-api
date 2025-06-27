# Multi-stage build for Rust application with PostgreSQL support
FROM rust:1.87-slim-bullseye AS builder

WORKDIR /app

# Install build dependencies including PostgreSQL client libraries
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Install sqlx-cli for database migrations
RUN cargo install sqlx-cli --no-default-features --features postgres

# Copy workspace files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Copy all crates for workspace build
COPY crates ./crates

# Copy migration files
COPY migrations ./migrations

# Build the application in release mode
RUN cargo build --release --bin app

# Runtime stage - minimal image for deployment
FROM debian:bullseye-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libpq5 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -ms /bin/bash appuser

# Switch to non-root user
USER appuser
WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /app/target/release/app /app/vectra-api

# Copy sqlx binary for migrations
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# Copy migration files
COPY --from=builder /app/migrations /app/migrations

# Copy startup script
COPY startup.sh /app/startup.sh

# Make startup script executable
USER root
RUN chmod +x /app/startup.sh
USER appuser

# Environment variables
ENV RUST_LOG=info
ENV PORT=5000

# Expose the port
EXPOSE 5000

# Health check endpoint
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:5000/health || exit 1

# Use startup script as entrypoint
ENTRYPOINT ["/app/startup.sh"]
