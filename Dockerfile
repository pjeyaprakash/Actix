# =========================
# Stage 1: Builder
# =========================
FROM rust:1.98-slim-bookworm AS builder

WORKDIR /usr/src/app

# Native dependencies required for compilation
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy dependency manifests first
COPY Cargo.toml Cargo.lock ./

# Cache dependencies
RUN mkdir src && \
    echo 'fn main() {}' > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy application source
COPY src ./src

# Build application
RUN cargo build --release

# =========================
# Stage 2: Runtime
# =========================
FROM gcr.io/distroless/cc-debian12:nonroot

WORKDIR /app

COPY --from=builder \
    /usr/src/app/target/release/billing_backend \
    /app/billing_backend

ENV RUST_LOG=warn \
    RUST_BACKTRACE=0

EXPOSE 8080

USER nonroot:nonroot

ENTRYPOINT ["/app/billing_backend"]

















## =========================
## Stage 1: Builder
## =========================
#FROM rust:1.98.0-alpine3.21 AS builder
#
#RUN apk add --no-cache \
#    musl-dev \
#    pkgconfig \
#    openssl-dev \
#    openssl-libs-static \
#    ca-certificates
#
## Install MUSL target
#RUN rustup target add x86_64-unknown-linux-musl
#
#WORKDIR /usr/src/app
#
## Copy dependency manifests first
#COPY Cargo.toml Cargo.lock ./
#
## Cache dependencies
#RUN mkdir src && \
#    echo 'fn main() {}' > src/main.rs && \
#    cargo build \
#        --release \
#        --target x86_64-unknown-linux-musl && \
#    rm -rf src
#
## Copy actual source
#COPY src ./src
#
## Build application
#RUN cargo build \
#    --release \
#    --target x86_64-unknown-linux-musl
#
## Strip binary
#RUN strip target/x86_64-unknown-linux-musl/release/billing_backend
#
#
## =========================
## Stage 2: Runtime
## =========================
#FROM alpine:3.21
#
#RUN apk add --no-cache \
#    ca-certificates \
#    tzdata \
#    openssl \
#    wget
#
## Non-root user
#RUN addgroup -g 1000 -S appgroup && \
#    adduser -u 1000 -S appuser -G appgroup
#
## Copy binary
#COPY --from=builder \
#    /usr/src/app/target/x86_64-unknown-linux-musl/release/billing_backend \
#    /usr/local/bin/billing_backend
#
#RUN chown appuser:appgroup /usr/local/bin/billing_backend && \
#    chmod 755 /usr/local/bin/billing_backend
#
#USER appuser
#
#ENV RUST_LOG=warn \
#    RUST_BACKTRACE=0
#
#EXPOSE 8080
#
#HEALTHCHECK \
#    --interval=30s \
#    --timeout=3s \
#    --start-period=5s \
#    --retries=3 \
#    CMD wget --no-verbose \
#        --tries=1 \
#        --spider \
#        http://localhost:8080/health || exit 1
#
#CMD ["billing_backend"]