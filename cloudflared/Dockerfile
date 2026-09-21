ARG BUILD_FROM
FROM $BUILD_FROM

ARG TARGETARCH
ARG TARGETVARIANT

RUN apk add --no-cache curl ca-certificates jq && \
    case "${TARGETARCH}${TARGETVARIANT}" in \
      amd64)   CF_ARCH="amd64" ;; \
      arm64)   CF_ARCH="arm64" ;; \
      armv7)   CF_ARCH="arm" ;; \
      *) echo "unsupported arch: ${TARGETARCH}${TARGETVARIANT}" && exit 1 ;; \
    esac && \
    curl -fsSL -o /usr/bin/cloudflared \
      "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-${CF_ARCH}" && \
    chmod +x /usr/bin/cloudflared

COPY run.sh /run.sh
RUN chmod +x /run.sh

CMD [ "/run.sh" ]
