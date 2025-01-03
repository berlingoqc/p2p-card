FROM rust:1.73 as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch
RUN cargo build --release || true

COPY src ./src

RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache libssl-dev

COPY --from=builder /usr/src/app/target/release/signaling /usr/local/bin/signaling

ENTRYPOINT ["/usr/local/bin/signaling"]
