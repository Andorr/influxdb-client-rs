use reqwest::{Client as HttpClient, Method, StatusCode, Url};

use crate::{
    models::{InfluxError, Precision, TimestampOptions},
    traits::PointSerialize,
};

/// Client for InfluxDB
#[derive(Clone)]
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
    /// Create an influxdb client with given host url and token.
    ///
    /// # Example
    /// ```
    /// use influxdb_client::Client;
    /// let client = Client::new("https://example.com:8086", "generated_token").unwrap();
    /// ```
    pub fn new(host: impl AsRef<str>, token: impl Into<String>) -> Result<Client, url::ParseError> {
        let host = reqwest::Url::parse(host.as_ref())?;

        Ok(Client {
            host,
            token: token.into(),
            client: HttpClient::default(),
            bucket: None,
            org: None,
            org_id: None,
            precision: Precision::NS,
            insert_to_stdout: false,
        })
    }

    /// Do not send request to influxdb but print to stdout. Useful for debugging
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
            .map(|p| match options.clone() {
                TimestampOptions::Use(t) => p.serialize_with_timestamp(Some(t)),
                TimestampOptions::FromPoint => p.serialize_with_timestamp(None),
                TimestampOptions::None => p.serialize(),
            })
            .collect::<Vec<String>>()
            .join("\n");

        let precision = self.precision.to_string();
        let write_query_params = [("precision", precision)];

        if self.insert_to_stdout {
            println!("{}", body);
            Ok(())
        } else {
            let response = self
                .new_request(Method::POST, "/api/v2/write")
                .query(&write_query_params)
                .body(body)
                .send()
                .await?;

            match response.status() {
                StatusCode::BAD_REQUEST => Err(InfluxError::InvalidSyntax(response)),
                StatusCode::UNAUTHORIZED => Err(InfluxError::InvalidCredentials(response)),
                StatusCode::FORBIDDEN => Err(InfluxError::Forbidden(response)),
                s if matches!(s.as_u16(), 400..=499 | 500..=500) => {
                    Err(InfluxError::Unknown(response))
                }
                _ => Ok(()),
            }
        }
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
}
