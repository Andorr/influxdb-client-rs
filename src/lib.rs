
pub mod models;
pub mod client;


#[cfg(test)]
mod tests {
    use super::models::Point;

    #[test]
    fn it_works() {
        let expected = "mem,host=host1 used_percent=23.43234543 1556896326";

        let point  = Point::new("mem")
            .tag("host", "host1")
            .field("used_percent", 23.43234543)
            .timestamp(1556896326);

        let actual = point.serialize();

        assert_eq!(actual, expected);
    }
}
