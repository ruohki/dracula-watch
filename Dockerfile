FROM clux/muslrust as cargo-build
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/dracula-watch
COPY ./ ./
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
RUN addgroup -g 1000 dracula
RUN adduser -D -s /bin/sh -u 1000 -G dracula dracula
WORKDIR /home/dracula/bin/
COPY --from=cargo-build /usr/src/dracula-watch/target/x86_64-unknown-linux-musl/release/dracula-watch .
RUN chown dracula:dracula dracula-watch
USER dracula
CMD ["./dracula-watch"]