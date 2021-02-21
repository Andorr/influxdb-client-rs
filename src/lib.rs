
pub mod models;
pub mod client;


#[cfg(test)]
mod tests {
    use super::client::{Client};
    use super::models::{Point};

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
            return
        } 

        assert!(true);
    }
}
