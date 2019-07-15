FROM rust:latest as build
COPY ./ ./
RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/udp /build-out

FROM debian:latest
COPY --from=build /build-out/udp /
CMD /udp

