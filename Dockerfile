FROM clux/muslrust:1.76.0-stable as builder
WORKDIR /usr/src/bilderna

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release && \
    mkdir -p /build-out && \
    cp target/x86_64-unknown-linux-musl/release/bilderna /build-out/


FROM scratch

COPY --from=builder /build-out/bilderna /bilderna

COPY ./assets ./assets

HEALTHCHECK --interval=5m --timeout=3s CMD /bilderna check

ENTRYPOINT ["/bilderna"]
