FROM ekidd/rust-musl-builder:latest as builder

COPY --chown=rust:rust Cargo.toml Cargo.lock sqlx-data.json ./
COPY --chown=rust:rust src ./src/
ENV SQLX_OFFLINE=true
RUN cargo build --release

RUN openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/email_server /usr/local/bin/
