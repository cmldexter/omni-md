# Stage 1: Build Environment (Debian-based for pure C-binding compilation safety)
FROM rust:1.80-slim-bookworm AS builder

# Install vital C-bindings, SSL protocols, and CMake for compiling libgit2 and sqlx
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libgit2-dev \
    cmake \
    git \
    sqlite3 \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Compile backend logic exclusively for release performance 
RUN cargo build --release

# Stage 2: Production Safe Container 
FROM debian:bookworm-slim

# Install only the absolute necessities (Git CLI, SQLite, and Curl for Web Probes)
RUN apt-get update && apt-get install -y \
    openssl \
    git \
    sqlite3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Easter Egg: Establish a highly restricted, non-root user named "genome"
RUN groupadd -r genome && useradd -r -g genome -d /app -M -s /sbin/nologin genome

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/md-relay-worker /app/md-relay-worker

# Ensure the 'genome' user exclusively owns the workspace so it can generate SQLite WAL lockfiles safely
RUN chown -R genome:genome /app

# Drop root privileges immediately. The container executes fully sandboxed going forward.
USER genome

# Mount bindings. Note the `data/` subdirectory for clean Volume mounting mechanics.
ENV DATABASE_URL="sqlite:///app/data/omni.db"
ENV RUST_LOG="info"

EXPOSE 8080

CMD ["./md-relay-worker"]
