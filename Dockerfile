FROM ekidd/rust-musl-builder:latest AS build

COPY . /home/rust/src
RUN cargo build --release

# ---------

FROM alpine:latest

RUN apk add --update --no-cache tini

COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/mqtt2influx /bin/mqtt2influx

ENTRYPOINT ["/sbin/tini", "--", "/bin/mqtt2influx"]
