ARG BUILD_FROM

FROM --platform=$BUILDPLATFORM ghcr.io/rust-cross/rust-musl-cross:x86_64-musl AS builder-amd64
FROM --platform=$BUILDPLATFORM ghcr.io/rust-cross/rust-musl-cross:aarch64-musl AS builder-arm64

FROM builder-${TARGETARCH} AS builder
WORKDIR /build
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release
RUN cp target/*/release/webhook-proxy /build/webhook-proxy

FROM $BUILD_FROM
RUN apk add --no-cache ca-certificates
COPY --from=builder /build/webhook-proxy /usr/bin/webhook-proxy
RUN chmod +x /usr/bin/webhook-proxy
CMD [ "/usr/bin/webhook-proxy" ]
