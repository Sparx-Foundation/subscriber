FROM docker.io/library/rust:slim-bullseye AS base


FROM base AS builder

WORKDIR /subscriber

COPY . .

RUN cargo build --release --locked

RUN rm -rf target/release/build \
    && rm -rf target/release/deps \
    && rm -rf target/release/incremental \
    && rm -rf target/release/.fingerprint

FROM debian:latest AS runner

RUN apt-get update \
    && apt-get install -y openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /subscriber/target/release/subscriber /usr/local/bin/subscriber

EXPOSE 8888

CMD ["subscriber"]
