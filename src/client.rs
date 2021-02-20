use std::collections::binary_heap::Iter;

use reqwest::{Client as HttpClient, Url};

use super::models::{Point};

struct Client {
    host: Url,
    token: String,
    client: HttpClient,
}

impl Client {

    pub fn new<T>(host: Url, token: T) -> Self 
    where
        T: Into<String>,
    {

        Client {
            host,
            token: token.into(),
            client: HttpClient::default(),
        }
    }

    /* pub fn insert_points<T: Iterator<Item = Point>>(self, points: T) {

        

        let mut req = self.client.get(None)
            .header("Content-Type", "text/plain")
            .header("Authorization", format!("{} {}", "Token ", self.token));
    } */

}