# MQTT2Influx

[![Lint](https://github.com/cquintana92/mqtt2influx/actions/workflows/lint.yaml/badge.svg)](https://github.com/cquintana92/mqtt2influx/actions/workflows/lint.yaml)
[![Tests](https://github.com/cquintana92/mqtt2influx/actions/workflows/tests.yaml/badge.svg)](https://github.com/cquintana92/mqtt2influx/actions/workflows/tests.yaml)
[![Release](https://github.com/cquintana92/mqtt2influx/actions/workflows/release.yaml/badge.svg)](https://github.com/cquintana92/mqtt2influx/actions/workflows/release.yaml)
[![Docker image](https://github.com/cquintana92/mqtt2influx/actions/workflows/docker-image.yaml/badge.svg)](https://github.com/cquintana92/mqtt2influx/actions/workflows/docker-image.yaml)

This repository contains the source code for `mqtt2influx`, a utility that helps you store events from a mqtt broker to an influxdb database.

Its main purpose is to read sensor readings coming from a [Zigbee2MQTT](https://github.com/koenkk/zigbee2mqtt). 

### Important notes

* Only compatible with InfluxDB v1 (for tests, version 1.5.4 is used).

### Configuration

An example configuration file can be found at the file [mqtt2influx.toml](./mqtt2influx.toml).

## License

Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org).
