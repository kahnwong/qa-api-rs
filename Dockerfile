FROM rust:1.93-trixie AS build
WORKDIR /app

COPY Cargo.lock Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY src src
RUN cargo build --release && \
    strip target/release/qa-api-rs && \
    cp ./target/release/qa-api-rs /qa-api-rs

# hadolint ignore=DL3007
FROM gcr.io/distroless/cc:nonroot AS deploy

WORKDIR /
COPY --from=build /qa-api-rs /

ENTRYPOINT ["/qa-api-rs"]
