FROM rust:1.75-bookworm as build-env
WORKDIR /app
COPY . /app
RUN rustup target add x86_64-unknown-linux-musl
RUN RUSTFLAGS='-C target-cpu=native' cargo build --profile min --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static-debian12:latest
COPY --from=build-env /app/target/x86_64-unknown-linux-musl/min/vbox-service-restarter-rs /
CMD ["./vbox-service-restarter-rs"]
