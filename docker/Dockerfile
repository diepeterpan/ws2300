# syntax=docker/dockerfile:1

FROM rust:bookworm AS builder

RUN apt-get update -yqq && \
    apt-get install -yqq --no-install-recommends libudev-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/ws2300

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release --bin ws2300

# Stage for exporting compiled binary directly to host ./bin folder:
# docker build -f docker/Dockerfile --target export-bin --output type=local,dest=./bin .
FROM scratch AS export-bin
COPY --from=builder /usr/src/ws2300/target/release/ws2300 /ws2300

# Runtime image for docker-compose deployment
FROM debian:bookworm-slim AS runtime

RUN apt-get update -yqq && \
    apt-get install -yqq --no-install-recommends libudev1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/ws2300/target/release/ws2300 /usr/local/bin/ws2300

EXPOSE 8080

ENV WS2300_CONFIG=/app/config/config.yaml
ENV RUST_LOG=info

ENTRYPOINT ["/usr/local/bin/ws2300"]
