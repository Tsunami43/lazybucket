FROM ubuntu:24.04

WORKDIR /app

ENV DEBIAN_FRONTEND=noninteractive

# System deps
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

# Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:$PATH"

# sqlx-cli for migrations
RUN cargo install sqlx-cli --no-default-features --features sqlite

# Build frontend
COPY frontend/package*.json ./frontend/
RUN cd frontend && npm ci

COPY frontend/ ./frontend/
RUN cd frontend && npm run build

# Build backend
COPY Cargo.toml Cargo.lock ./
COPY migrations/ migrations/
COPY src/ src/

RUN sqlx database create --database-url sqlite://build.db && \
    sqlx migrate run --database-url sqlite://build.db && \
    DATABASE_URL=sqlite://build.db cargo build --release && \
    rm -f build.db

RUN mkdir -p /app/storage

VOLUME ["/app/storage"]

EXPOSE 8000

CMD ["./target/release/lazybucket"]
