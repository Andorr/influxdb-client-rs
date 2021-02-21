# 🦀 InfluxDB Rust Client

[![tests](https://github.com/Andorr/influxdb-client-rs/actions/workflows/test.yml/badge.svg)](https://github.com/Andorr/influxdb-client-rs/actions/workflows/test.yml)

![docs](https://img.shields.io/endpoint?color=orange&label=docs&url=https%3A%2F%2Fandorr.github.io%2Finfluxdb-client-rs%2Finfluxdb_client_rs%2Findex.htmlhttps%3A%2F%2Fandorr.github.io%2Finfluxdb-client-rs%2Finfluxdb_client_rs%2Findex.html)

* https://docs.influxdata.com/influxdb/v2.0/api/#operation/PostWrite


```rust
use influxdb_client::{Client, Precision, Point, points};


let mut client = Client::new("http://localhost:8086", "...")
    .with_bucket("tradely")
    .with_precision(Precision::ms);


client.ping()

let point = Point::new("measurement")
    .tag("ticker", "GME")
    .field("price", 2.23)
    .timestamp(1613832794);

client.insert_point(point);

client.insert_points(points!(point, point));

```