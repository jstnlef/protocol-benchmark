FROM rust:latest as build
COPY ./ /app
WORKDIR /app
RUN cargo build --release --bin udp-client
RUN mkdir -p /build-out
RUN cp target/release/udp-client /build-out

FROM debian:latest
COPY --from=build /build-out/udp-client /app/udp-client
CMD /app/udp-client
