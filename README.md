# 🦀 InfluxDB Rust Client

[![tests](https://github.com/Andorr/influxdb-client-rs/actions/workflows/test.yml/badge.svg)](https://github.com/Andorr/influxdb-client-rs/actions/workflows/test.yml)
[![docs](https://img.shields.io/badge/docs-0.1.0-orange)](https://andorr.github.io/influxdb-client-rs/influxdb_client_rs/index.html)


**NB! - This library is still in development and is not ready for production!**

An unofficial client-library for InfluxDB v2. 

## ⬇️ Installation
```
influxdb-client = "0.1.3"
```

## ❤️‍🔥 Usage

### Insert by building a Point
```rust
use influxdb_client::{Client, Point, Precision, TimestampOptions};

let client = Client::new("http://localhost:8086", "token")
    .with_org_id("168f31904923e853")
    .with_bucket("tradely")
    .with_precision(Precision::MS);

let point = Point::new("test")
    .tag("ticker", "GME")
    .field("price", 420.69)
    .timestamp(1614956250000);

let points = vec![point];

// Insert with the timestamp from the point (1614956250000)
let result = client.insert_points(&points, TimestampOptions::FromPoint).await;

```

### Insert using a struct
```rust

use influxdb_client::{Client, Precision, PointSerialize, TimestampOptions, Timestamp};
use influxdb_client::derives::PointSerialize;

let client = Client::new("http://localhost:8086", "token")
    .with_org_id("168f31904923e853")
    .with_bucket("tradely")
    .with_precision(Precision::MS);

#[derive(PointSerialize)]
#[point(measurement = "test")]
struct Ticker {
    #[point(tag)]
    ticker: String,
    #[point(field = "tickerPrice")]
    price: f64,
    #[point(timestamp)]
    timestamp: Timestamp,
}

let point = Ticker {
    ticker: String::from("GME"),
    price: 420.69,
    timestamp: Timestamp::from(1614956250000)
};

let points = vec![point];

// Insert without timestamp - InfluxDB will automatically set the timestamp
let result = client.insert_points(&points, TimestampOptions::None).await;

```

## 🪧 TODO
This todolist is still in progress and will be expanded in the future.

- [x] Implement insertion into InfluxDB from client
- [x] Implement procedural macro for implementing PointSerialize
- [ ] Implement querying
- [ ] Implement other important things 