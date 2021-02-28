# 🦀 InfluxDB Rust Client

[![tests](https://github.com/Andorr/influxdb-client-rs/actions/workflows/test.yml/badge.svg)](https://github.com/Andorr/influxdb-client-rs/actions/workflows/test.yml)
[![docs](https://img.shields.io/badge/docs-0.1.0-orange)](https://andorr.github.io/influxdb-client-rs/influxdb_client_rs/index.html)


**NB! - This library is still in development and is not ready for production!**

An unofficial client-library for InfluxDB v2. 

## Usage

```rust
use influxdb_client::{Client, Point, InsertOptions};


let mut client = Client::new("http://localhost:8086", "...")
    .with_bucket("tradely");


let point = Point::new("measurement")
    .tag("ticker", "GME")
    .field("price", 2.23)
    .timestamp(1613832794);

let points = vec![point];

client.insert_points(points, InsertOptions::WithTimestamp(None));

```