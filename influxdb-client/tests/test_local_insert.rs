use influxdb_client::{Client, Point, TimestampOptions};

#[tokio::test]
#[ignore = "ignore local test"]
async fn test_local_insert() {
    let client = Client::new(
        "http://127.0.0.1:8086",
        "4mK1cnEIhEZyRNXDizIjdoujogPX4Js1igojxuxMdin-gXZgq9QDG0XLGxbyFjcbnPDjRi0uBINqJ24EgIbozQ==",
    )
    .unwrap()
    .with_bucket("test")
    .with_org("ddpanel");

    let point = Point::new("some-measurement")
        .tag("tag name", "tag value")
        .field("field name", "field value");
    client
        .insert_points(&[point], TimestampOptions::None)
        .await
        .unwrap();
}
