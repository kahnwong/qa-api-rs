FROM rust:1.81 AS build

WORKDIR /app

COPY Cargo.lock Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY src src
RUN cargo build --release && \
    strip target/release/qa-api-rs && \
    cp ./target/release/qa-api-rs /qa-api-rs

# hadolint ignore=DL3007
FROM rust:1.81 AS deploy
COPY --from=build /qa-api-rs /

CMD ["/qa-api-rs"]
