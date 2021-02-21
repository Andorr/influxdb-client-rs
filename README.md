# 🦀 InfluxDB Rust Client

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