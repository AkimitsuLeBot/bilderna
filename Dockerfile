# Builder phase
FROM rust:1.50 as builder
WORKDIR /usr/src/bilderna

COPY . .

RUN cargo install --path .

# Bundle phase
FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/bilderna /usr/local/bin/bilderna

COPY ./assets ./assets

CMD ["bilderna"]