FROM rust:latest as build
COPY ./ /app
WORKDIR /app
RUN cargo build --release --bin udp-server
RUN mkdir -p /build-out
RUN cp target/release/udp-server /build-out

FROM debian:latest
COPY --from=build /build-out/udp-server /app/udp-server
CMD /app/udp-server
