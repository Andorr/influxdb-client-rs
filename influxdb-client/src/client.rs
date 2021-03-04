use reqwest::{Client as HttpClient, Method, Url};

use std::error::Error;

use crate::{
    models::{InfluxError, Precision, TimestampOptions},
    traits::PointSerialize,
};

/// Client for InfluxDB
pub struct Client {
    host: Url,
    token: String,
    client: HttpClient,
    bucket: Option<String>,
    org: Option<String>,
    org_id: Option<String>,
    precision: Precision,

    insert_to_stdout: bool,
}

impl Client {
    pub fn new<T>(host: T, token: T) -> Client
    where
        T: Into<String>,
    {
        let host_url = reqwest::Url::parse(&host.into()[..]).unwrap();

        Client {
            host: host_url,
            token: token.into(),
            client: HttpClient::default(),
            bucket: None,
            org: None,
            org_id: None,
            precision: Precision::NS,
            insert_to_stdout: false,
        }
    }

    pub fn insert_to_stdout(mut self) -> Self {
        self.insert_to_stdout = true;
        self
    }

    pub fn with_bucket<T: Into<String>>(mut self, bucket: T) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    pub fn with_org<T: Into<String>>(mut self, org: T) -> Self {
        self.org = Some(org.into());
        self
    }

    pub fn with_org_id<T: Into<String>>(mut self, org_id: T) -> Self {
        self.org_id = Some(org_id.into());
        self
    }

    pub fn with_precision(mut self, precision: Precision) -> Self {
        self.precision = precision;
        self
    }

    pub fn precision(&self) -> &str {
        self.precision.to_string()
    }

    pub async fn insert_points<'a, I: IntoIterator<Item = &'a (impl PointSerialize + 'a)>>(
        &self,
        points: I,
        options: TimestampOptions,
    ) -> Result<(), InfluxError> {
        let body = points
            .into_iter()
            .map(|p| {
                format!(
                    "{}",
                    match options.clone() {
                        TimestampOptions::Use(t) => p.serialize_with_timestamp(Some(t)),
                        TimestampOptions::FromPoint => p.serialize_with_timestamp(None),
                        TimestampOptions::None => p.serialize(),
                    }
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let precision = self.precision.to_string();
        let write_query_params = [("precision", precision)];

        if self.insert_to_stdout {
            println!("{}", body);
        } else {
            let result = self
                .new_request(Method::POST, "/api/v2/write")
                .query(&write_query_params)
                .body(body)
                .send()
                .await
                .unwrap()
                .error_for_status();

            if let Err(err) = result {
                let status = err.status().unwrap().as_u16();
                return Err(Client::status_to_influxerror(status, Box::new(err)));
            }
        }

        Ok(())
    }

    fn new_request(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        // Build query params
        let mut query_params = Vec::<(&str, String)>::new();
        if let Some(bucket) = &self.bucket {
            query_params.push(("bucket", bucket.clone()));
        }

        if let Some(org) = &self.org {
            query_params.push(("org", org.clone()));
        } else if let Some(org_id) = &self.org_id {
            query_params.push(("orgID", org_id.clone()));
        }

        // Build default request
        let mut url = self.host.clone();
        url.set_path(path);

        self.client
            .request(method, url)
            .header("Content-Type", "text/plain")
            .header("Authorization", format!("{} {}", "Token", self.token))
            .query(&query_params)
    }

    fn status_to_influxerror(status: u16, err: Box<dyn Error>) -> InfluxError {
        match status {
            400 => InfluxError::InvalidSyntax(err.to_string()),
            401 => InfluxError::InvalidCredentials(err.to_string()),
            403 => InfluxError::Forbidden(err.to_string()),
            _ => InfluxError::Unknown(err.to_string()),
        }
    }
}
