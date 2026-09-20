ARG BUILD_FROM
FROM rust:1-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /build
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM $BUILD_FROM
RUN apk add --no-cache ca-certificates
COPY --from=builder /build/target/release/webhook-proxy /usr/bin/webhook-proxy
RUN chmod +x /usr/bin/webhook-proxy
CMD [ "/usr/bin/webhook-proxy" ]
