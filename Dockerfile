FROM rust:1.75-bookworm as build-env
WORKDIR /app
COPY . /app
RUN RUSTFLAGS='-C target-cpu=native' cargo build --profile min

FROM gcr.io/distroless/static-debian12:latest
COPY --from=build-env /app/target/min/vbox-service-restarter-rs /
CMD ["/vbox-service-restarter-rs"]
