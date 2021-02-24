pub mod client;
pub mod models;
pub mod traits;

#[cfg(test)]
mod tests {
    use super::client::Client;
    use super::models::Point;

    use super::traits::PointSerialize;
    use derives::PointSerialize;

    #[test]
    fn test_derive_write() {
        #[derive(PointSerialize)]
        #[point(measurement = "test")]
        struct Test {
            #[point(tag = "notTicker")]
            ticker: String,
            #[point(tag = "notTicker2")]
            ticker2: String,
            #[point(field = "notPrice")]
            price: f32,
            #[point(field = "notPrice2")]
            price2: String,
            #[point(timestamp)]
            data: String,
        }
        let result = Test {
            ticker: "GME".to_string(),
            ticker2: "!GME".to_string(),
            price: 0.32,
            price2: "Hello world".to_string(),
            data: "321321321".to_string(),
        }
        .serialize();
        println!("Wow, very serialized: {}", result);
        assert_eq!(
            "test,notTicker=GME,notTicker2=!GME notPrice=0.32,notPrice2=\"Hello world\""
                .to_string(),
            result
        );
    }

    #[test]
    fn test_client_write() {
        let client = Client::new(
            "http://localhost:8086", 
            "7CDkEyzbTR6LcPSmFzZ-3fIOR69LHZS-q45hThmJtoorCwcMH-I8GkZnPu10sDh7TYUb4kwutarpKBMMN4_nXg=="
        )
            .with_bucket("tradely")
            .with_org_id("168f31904923e853");

        let point = Point::new("test")
            .tag("ticker", "GME")
            .field("price", 420.69);

        let result = tokio_test::block_on(client.insert_points(point));
        if let Err(err) = result {
            println!("{:?}", err);
            assert!(false);
            return;
        }

        assert!(true);
    }
}
