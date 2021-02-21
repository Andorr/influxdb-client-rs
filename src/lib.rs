
pub mod models;
pub mod client;


#[cfg(test)]
mod tests {

    use super::client::{Client};
    use super::models::{Point};
    use mockito::{Matcher};

    #[test]
    fn test_client_write() {
        let api_key = "TEST_API_KEY";


        let mock = mockito::mock("POST", "/api/v2/write")
            .with_status(201)
            .match_header("content-type", "text/plain")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("bucket".into(), "tradely".into()),
                Matcher::UrlEncoded("orgID".into(), "168f31904923e853".into())
            ]))
            .match_body("test,ticker=GME price=420.69 1613925577")
            .expect(1)
            .create();



        let client = Client::new(
            mockito::server_url(), 
            String::from(api_key)
        )
            .with_bucket("tradely")
            .with_org_id("168f31904923e853");
        

        let point = Point::new("test")
            .tag("ticker", "GME")
            .field("price", 420.69)
            .timestamp(1613925577);


        let points = vec![point]; 
        let result = tokio_test::block_on(client.insert_points(&points));
        
        if let Err(err) = result {
            println!("{:?}", err);
            assert!(false);
            return
        } 

        mock.assert();
    }
}
