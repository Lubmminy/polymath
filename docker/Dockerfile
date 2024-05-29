FROM rust:alpine AS builder

RUN USER=root cargo new --bin polymath
WORKDIR /polymath

ENV     RUSTFLAGS="-C target-feature=-crt-static"
RUN     apk add -q --update-cache --no-cache build-base openssl-dev musl pkgconfig protobuf-dev

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./crates ./crates
COPY ./extensions ./extensions

RUN cargo build --release

FROM alpine:3 AS runtime

RUN apk add --no-cache libgcc

RUN addgroup -S appgroup && adduser -S rust -G appgroup
USER rust

COPY --from=builder /polymath/target/release/polymath /bin/polymath

CMD ["./bin/polymath"]