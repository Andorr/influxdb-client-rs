use std::io::Write;

use flate2::write::GzEncoder;
use flate2::Compression;
use mockito::Matcher;

use influxdb_client::{timestamp, Client, Point, Precision, Timestamp, TimestampOptions};

#[test]
fn test_client_write_plain() {
    let api_key = "TEST_API_KEY";

    let mock = mockito::mock("POST", "/api/v2/write")
        .with_status(201)
        .match_header("content-type", "text/plain")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("bucket".into(), "tradely".into()),
            Matcher::UrlEncoded("orgID".into(), "168f31904923e853".into()),
            Matcher::UrlEncoded("precision".into(), "ms".into()),
        ]))
        .match_body("test,ticker=GME price=420.69 1613925577")
        .expect(1)
        .create();

    let client = Client::new(mockito::server_url(), String::from(api_key))
        .unwrap()
        .with_bucket("tradely")
        .with_org_id("168f31904923e853")
        .with_precision(Precision::MS);

    let point = Point::new("test")
        .tag("ticker", "GME")
        .field("price", 420.69)
        .timestamp(1613925577);

    let points: Vec<Point> = vec![point];
    let result = tokio_test::block_on(client.insert_points(&points, timestamp!(1613925577)));

    assert!(result.is_ok());

    mock.assert();
}

#[test]
fn test_client_write_compressed() {
    let api_key = "TEST_API_KEY";

    let body: Vec<u8> = vec![
        31, 139, 8, 0, 0, 0, 0, 0, 0, 255, 1, 39, 0, 216, 255, 116, 101, 115, 116, 44, 116, 105,
        99, 107, 101, 114, 61, 71, 77, 69, 32, 112, 114, 105, 99, 101, 61, 52, 50, 48, 46, 54, 57,
        32, 49, 54, 49, 51, 57, 50, 53, 53, 55, 55, 179, 79, 127, 46, 39, 0, 0, 0,
    ];
    let mock = mockito::mock("POST", "/api/v2/write")
        .with_status(201)
        .match_header("content-type", "text/plain")
        .match_header("content-encoding", "gzip")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("bucket".into(), "tradely".into()),
            Matcher::UrlEncoded("orgID".into(), "168f31904923e853".into()),
            Matcher::UrlEncoded("precision".into(), "ms".into()),
        ]))
        .match_body(body)
        .expect(1)
        .create();

    let client = Client::new(mockito::server_url(), String::from(api_key))
        .unwrap()
        .with_bucket("tradely")
        .with_org_id("168f31904923e853")
        .with_precision(Precision::MS)
        .with_compression(6u32);

    let point = Point::new("test")
        .tag("ticker", "GME")
        .field("price", 420.69)
        .timestamp(1613925577);

    let points: Vec<Point> = vec![point];
    let result = tokio_test::block_on(client.insert_points(&points, timestamp!(1613925577)));

    assert!(result.is_ok());

    mock.assert();
}
