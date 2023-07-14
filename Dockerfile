FROM clux/muslrust:stable AS build

WORKDIR /volume
COPY . /volume
RUN cargo build --release

# ---------

FROM alpine:latest

RUN apk add --update --no-cache tini

COPY --from=build /volume/target/x86_64-unknown-linux-musl/release/mqtt2influx /bin/mqtt2influx

ENTRYPOINT ["/sbin/tini", "--", "/bin/mqtt2influx"]
