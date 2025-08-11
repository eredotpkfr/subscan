FROM rust:1.89-slim-bookworm AS builder

WORKDIR /builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
    chromium \
    libssl-dev \
    pkg-config \
    tini \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN cargo build --release

# hadolint ignore=DL3007
FROM gcr.io/distroless/cc-debian12:latest

ENV SUBSCAN_CHROME_PATH=/usr/lib/chromium/chromium

# Copy libs from builder
COPY --from=builder /usr/lib /usr/lib
COPY --from=builder /usr/share /usr/share
# Copy required binaries
COPY --from=builder /usr/bin/tini /bin/tini
COPY --from=builder /builder/target/release/subscan /bin/subscan

WORKDIR /data

ENTRYPOINT ["tini", "--", "subscan"]
