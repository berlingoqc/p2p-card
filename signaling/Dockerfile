FROM rust:1.75 as builder

WORKDIR /usr/src/app

COPY Cargo.toml ./
COPY src ./src

RUN cargo fetch

RUN cargo build --release

FROM alpine:latest

COPY --from=builder /usr/src/app/target/release/signaling /usr/local/bin/signaling

ENTRYPOINT ["/usr/local/bin/signaling"]
