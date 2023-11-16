# Builder phase
FROM rust:1.74-bookworm as builder
WORKDIR /usr/src/bilderna

COPY . .

RUN cargo install --path .


# Bundle phase
FROM debian:bookworm

RUN apt-get update && apt-get install -y curl

COPY --from=builder /usr/local/cargo/bin/bilderna /usr/local/bin/bilderna

COPY ./assets ./assets

HEALTHCHECK --interval=5m --timeout=3s \
    CMD curl -f http://localhost:3000/ping || exit 1

CMD ["bilderna"]
