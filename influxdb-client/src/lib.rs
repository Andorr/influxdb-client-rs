pub mod client;
pub mod models;
pub mod traits;
pub mod macros;

#[cfg(test)]
mod tests {

    use super::client::{Client, TimestampOptions, Precision};
    use super::models::{Point, Timestamp};
    use super::traits::PointSerialize;
    use super::timestamp;
    use mockito::Matcher;

    use influxdb_derives::PointSerialize;

    #[test]
    fn test_derive_serialize() {
        #[derive(PointSerialize)]
        #[point(measurement = "test")]
        struct Test {
            #[point(tag = "notTicker")]
            ticker: String,
            #[point(tag = "notTicker2")]
            ticker2: String,
            #[point(field = "notPrice")]
            price: f32,
            #[point(field)]
            price2: String,
            #[point(timestamp)]
            data: Timestamp,
        }

        let result = Test {
            ticker: "GME".to_string(),
            ticker2: "!GME".to_string(),
            price: 0.32,
            price2: "Hello world".to_string(),
            data: Timestamp::from("321321321"),
        }
        .serialize();

        assert_eq!(
            "test,notTicker=GME,notTicker2=!GME notPrice=0.32,price2=\"Hello world\"".to_string(),
            result
        );
    }

    #[test]
    fn test_derive_serialize_with_timestamp() {
        #[derive(PointSerialize)]
        #[point(measurement = "test")]
        struct Test {
            #[point(tag = "notTicker")]
            ticker: String,
            #[point(tag = "notTicker2")]
            ticker2: String,
            #[point(field = "notPrice")]
            price: f32,
            #[point(field)]
            price2: String,
            #[point(timestamp)]
            data: Timestamp,
        }

        let data = Test {
            ticker: "GME".to_string(),
            ticker2: "!GME".to_string(),
            price: 0.32,
            price2: "Hello world".to_string(),
            data: Timestamp::from("321321321"),
        };
        let result = data.serialize_with_timestamp(None);
        assert_eq!(
            "test,notTicker=GME,notTicker2=!GME notPrice=0.32,price2=\"Hello world\" 321321321"
                .to_string(),
            result
        );

        let result_2 = data.serialize_with_timestamp(Some(Timestamp::from(420)));
        assert_eq!(
            "test,notTicker=GME,notTicker2=!GME notPrice=0.32,price2=\"Hello world\" 420"
                .to_string(),
            result_2
        );
    }

    #[test]
    fn test_client_write() {
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
            .with_bucket("tradely")
            .with_org_id("168f31904923e853")
            .with_precision(Precision::MS);

        let point = Point::new("test")
            .tag("ticker", "GME")
            .field("price", 420.69)
            .timestamp(1613925577);

        let points: Vec<Point> = vec![point];
        let result = tokio_test::block_on(
            client.insert_points(&points, timestamp!(1613925577)),
        );

        assert!(result.is_ok());

        mock.assert();
    }
}
