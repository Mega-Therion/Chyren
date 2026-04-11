# OmegA-Next Core Dockerfile (Optimized)
FROM rust:1.88-slim-bookworm AS builder
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev gcc && rm -rf /var/lib/apt/lists/*

# Optimize for caching dependencies
COPY Cargo.toml Cargo.lock ./
COPY omega-*/Cargo.toml ./
COPY omega-core/Cargo.toml ./omega-core/
COPY omega-aegis/Cargo.toml ./omega-aegis/
COPY omega-aeon/Cargo.toml ./omega-aeon/
COPY omega-adccl/Cargo.toml ./omega-adccl/
COPY omega-myelin/Cargo.toml ./omega-myelin/
COPY omega-dream/Cargo.toml ./omega-dream/
COPY omega-metacog/Cargo.toml ./omega-metacog/
COPY omega-worldmodel/Cargo.toml ./omega-worldmodel/
COPY omega-integration/Cargo.toml ./omega-integration/
COPY omega-spokes/Cargo.toml ./omega-spokes/
COPY omega-telemetry/Cargo.toml ./omega-telemetry/
COPY omega-conductor/Cargo.toml ./omega-conductor/
COPY omega-eval/Cargo.toml ./omega-eval/
COPY omega-cli/Cargo.toml ./omega-cli/

RUN mkdir -p omega-core/src omega-aegis/src omega-aeon/src omega-adccl/src omega-myelin/src \
    omega-dream/src omega-metacog/src omega-worldmodel/src omega-integration/src \
    omega-spokes/src omega-telemetry/src omega-conductor/src omega-eval/src omega-cli/src && \
    echo "pub fn main() {}" > omega-core/src/lib.rs && \
    echo "pub fn main() {}" > omega-aegis/src/lib.rs && \
    echo "pub fn main() {}" > omega-aeon/src/lib.rs && \
    echo "pub fn main() {}" > omega-adccl/src/lib.rs && \
    echo "pub fn main() {}" > omega-myelin/src/lib.rs && \
    echo "pub fn main() {}" > omega-dream/src/lib.rs && \
    echo "pub fn main() {}" > omega-metacog/src/lib.rs && \
    echo "pub fn main() {}" > omega-worldmodel/src/lib.rs && \
    echo "pub fn main() {}" > omega-integration/src/lib.rs && \
    echo "pub fn main() {}" > omega-spokes/src/lib.rs && \
    echo "pub fn main() {}" > omega-telemetry/src/lib.rs && \
    echo "pub fn main() {}" > omega-conductor/src/lib.rs && \
    echo "pub fn main() {}" > omega-eval/src/lib.rs && \
    echo "fn main() {}" > omega-cli/src/main.rs && \
    echo "pub fn main() {}" > omega-cli/src/lib.rs

RUN cargo build --release -p omega-cli --bin chyren && \
    rm -rf omega-*/src

# Copy actual source code
COPY . .

# Build the release binary
RUN cargo build --release -p omega-cli --bin chyren

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/chyren /usr/local/bin/chyren

EXPOSE 8080
ENV CHYREN_API_HOST=0.0.0.0
ENV CHYREN_API_PORT=8080
# Default command for the core image (CLI mode)
ENTRYPOINT ["chyren"]
CMD ["--help"]
