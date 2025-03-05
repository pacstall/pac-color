FROM rust:slim as build-env

WORKDIR /app
COPY . /app
RUN cargo build --release --locked

FROM gcr.io/distroless/cc

COPY --from=build-env /app/target/release/pac-color /
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["./pac-color"]
