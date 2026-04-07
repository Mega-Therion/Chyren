# Builder Stage
FROM rust:1.77-slim-bookworm as builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev gcc && rm -rf /var/lib/apt/lists/*

# Copy the entire workspace
COPY . .

# Build the release binary
RUN cargo build --release --bin chyren_api

# Runtime Stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder
COPY --from=builder /usr/src/app/target/release/chyren_api /usr/local/bin/chyren_api

# Default environment configurations
ENV CHYREN_API_HOST=0.0.0.0
ENV CHYREN_API_PORT=8080

# Expose port
EXPOSE 8080

# Command to run the API
CMD ["chyren_api"]
