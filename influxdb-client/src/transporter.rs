use std::future::{Future};

use reqwest::{Method, blocking::{Client as HttpClient}};

use crate::{InfluxError, client::{ClientOptions}};

pub trait Transporter {
    fn insert(&self, options: ClientOptions, body: &Vec<String>, params: &Vec<(&str, &str)>) -> Result<(), InfluxError>;
}


pub struct InfluxDBTransporter {
    client: HttpClient
}

impl InfluxDBTransporter {
    pub fn new() -> Self {
        InfluxDBTransporter {
            client: HttpClient::default(),
        }
    }
}

impl InfluxDBTransporter {

    fn new_request(&self, options: &ClientOptions, method: Method, path: &str) -> reqwest::blocking::RequestBuilder {
        // Build query params
        let mut query_params = Vec::<(&str, String)>::new();
        if let Some(bucket) = &options.bucket {
            query_params.push(("bucket", bucket.clone()));
        }

        if let Some(org) = &options.org {
            query_params.push(("org", org.clone()));
        } else if let Some(org_id) = &options.org_id {
            query_params.push(("orgID", org_id.clone()));
        }

        // Build default request
        let mut url = options.host.clone();
        url.set_path(path);


        self.client
            .request(method, url)
            .header("Content-Type", "text/plain")
            .header("Authorization", format!("{} {}", "Token", options.token))
            .query(&query_params)
    }

    fn status_to_influxerror(status: u16, err: Box<dyn std::error::Error>) -> InfluxError {
        match status {
            400 => InfluxError::InvalidSyntax(err.to_string()),
            401 => InfluxError::InvalidCredentials(err.to_string()),
            403 => InfluxError::Forbidden(err.to_string()),
            _ => InfluxError::Unknown(err.to_string()),
        }
    }
}

impl Transporter for InfluxDBTransporter {
    fn insert(&self, options: ClientOptions, body: &Vec<String>, params: &Vec<(&str, &str)>) -> Result<(), InfluxError> {
        let precision = options.precision.to_string();
        let write_query_params = vec![("precision", precision)];

     
        let result = self
            .new_request(&options, Method::POST, "/api/v2/write")
            .query(&write_query_params)
            .body(body.join("\n"))
            .send()
            .unwrap()
            .error_for_status();

        if let Err(err) = result {
            let status = err.status().unwrap().as_u16();
            return Err(InfluxDBTransporter::status_to_influxerror(status, Box::new(err)));
        }
        Ok(())
    }
}