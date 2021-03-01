# InfluxDB Client

```rust
let client = Client::new("...", "...")
    .with_bucket("..")
    .with_precision(Precision::MS);


#[derive(PointSerialize)]
#[point(measurement = "test")]
struct MyPoint {
    [point(tag)]
    ticker: String,

    [point(field)]
    price: f64,

    [point(timestamp)]
    timestamp: Timestamp,
}

let point = MyPoint {
    ticker: "GME".to_string(),
    price: 420.9,
    timestamp: Timestamp::from(0), 
};


let points = vec![point];

// InfluxDB decides the timestamp
client.insert_points(points, TimestampOptions::None);

// Timestamp 101 will be used
client.insert_points(points, TimestampOptions::Use(101));

// Timestamp 0 will be used
client.insert_points(points, TimestampOptions::FromPoint);


let result = client.query<MyPoint>("SELECT * FROM food");

```