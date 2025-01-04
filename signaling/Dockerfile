FROM rust:1.78.0-buster as builder

WORKDIR /usr/src/app

COPY Cargo.toml ./
COPY src ./src

RUN cargo fetch

RUN cargo build --release

FROM debian:bullseye-slim
LABEL org.opencontainers.image.source https://github.com/berlingoqc/p2p-card

RUN apt install libc6
COPY --from=builder /usr/src/app/target/release/signaling /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/signaling"]
