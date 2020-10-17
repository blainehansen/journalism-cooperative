FROM ekidd/rust-musl-builder:latest as builder

COPY --chown=rust:rust Cargo.toml Cargo.lock sqlx-data.json ./
COPY --chown=rust:rust src ./src/
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/email_server /usr/local/bin/
EXPOSE 5050
CMD ["/usr/local/bin/email_server"]
