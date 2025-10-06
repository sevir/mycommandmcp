FROM --platform=$BUILDPLATFORM rust:1.90 AS builder
WORKDIR /app
RUN cargo install cargo-auditable

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY src ./src
RUN cargo auditable build --release --locked

FROM debian:13-slim

LABEL org.opencontainers.image.authors="jose@sevir.org" \
    org.opencontainers.image.url="https://github.com/sevir/mycommandmcp" \
    org.opencontainers.image.source="https://github.com/sevir/mycommandmcp" \
    org.opencontainers.image.vendor="github.com/sevir/mycommandmcp" \
    io.modelcontextprotocol.server.name="io.github.sevir/mycommandmcp"

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/mycommandmcp /usr/local/bin/mycommandmcp
ENTRYPOINT ["/usr/local/bin/mycommandmcp"]
