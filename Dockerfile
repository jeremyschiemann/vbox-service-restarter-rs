FROM rust:1.75.0 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/vbox-service-restarter-rs /
CMD ["./vbox-service-restarter-rs"]