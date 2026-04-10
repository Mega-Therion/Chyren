# Multi-stage Dockerfile for Chyren
# Optimized for production deployment

# ============================================
# Stage 1: Builder - Rust compilation
# ============================================
FROM rust:1.75-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    cmake \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /build

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./
COPY chyren_cli/Cargo.toml ./chyren_cli/
COPY chyren_py/Cargo.toml ./chyren_py/
COPY core/Cargo.toml ./core/

# Create dummy source files to cache dependencies
RUN mkdir -p chyren_cli/src chyren_py/src core/src && \
    echo "fn main() {}" > chyren_cli/src/main.rs && \
    echo "fn main() {}" > chyren_py/src/lib.rs && \
    echo "fn main() {}" > core/src/lib.rs

# Build dependencies (cached layer)
RUN cargo build --release && \
    rm -rf chyren_cli/src chyren_py/src core/src

# Copy actual source code
COPY . .

# Build the application
RUN cargo build --release --bin chyren

# Strip debug symbols to reduce binary size
RUN strip /build/target/release/chyren

# ============================================
# Stage 2: Python Environment
# ============================================
FROM python:3.11-slim-bookworm AS python-builder

WORKDIR /app

# Install Python build dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Copy Python requirements
COPY requirements.txt .

# Install Python dependencies
RUN pip install --no-cache-dir --user -r requirements.txt

# ============================================
# Stage 3: Final Runtime Image
# ============================================
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 -s /bin/bash chyren

# Set working directory
WORKDIR /app

# Copy compiled Rust binary from builder
COPY --from=builder /build/target/release/chyren /usr/local/bin/

# Copy Python environment
COPY --from=python-builder /root/.local /home/chyren/.local

# Copy application files
COPY --chown=chyren:chyren . .

# Set PATH for Python user packages
ENV PATH=/home/chyren/.local/bin:$PATH

# Environment variables
ENV RUST_LOG=info
ENV CHYREN_HOME=/app
ENV CHYREN_DATA=/data

# Create data directory
RUN mkdir -p /data && chown -R chyren:chyren /data

# Switch to non-root user
USER chyren

# Expose default ports
# 8080: HTTP API
# 8081: WebSocket
# 8082: gRPC (future)
EXPOSE 8080 8081 8082

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command
CMD ["chyren", "serve", "--host", "0.0.0.0", "--port", "8080"]

# ============================================
# Build & Run Instructions
# ============================================
# Build:
#   docker build -t chyren:latest .
#
# Run:
#   docker run -d \
#     -p 8080:8080 \
#     -p 8081:8081 \
#     -v $(pwd)/data:/data \
#     -e RUST_LOG=debug \
#     --name chyren \
#     chyren:latest
#
# Development (with volume mount):
#   docker run -it \
#     -p 8080:8080 \
#     -v $(pwd):/app \
#     -v $(pwd)/data:/data \
#     chyren:latest bash
# ============================================
