# Builder phase
FROM rust:1.71 as builder
WORKDIR /usr/src/bilderna

COPY . .

RUN cargo install --path .


# Bundle phase
FROM debian:buster-slim

RUN apt-get update && apt-get install -y curl

COPY --from=builder /usr/local/cargo/bin/bilderna /usr/local/bin/bilderna

COPY ./assets ./assets

HEALTHCHECK --interval=5m --timeout=3s \
    CMD curl -f http://localhost:3000/ping || exit 1

CMD ["bilderna"]
