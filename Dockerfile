FROM rust:1.50 as builder
WORKDIR /usr/src/bilderna

COPY ./src ./src
COPY ./assets ./assets
COPY Cargo.toml Cargo.lock ./

RUN cargo install --path .

FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/bilderna /usr/local/bin/bilderna

COPY ./assets ./assets

CMD ["bilderna"]