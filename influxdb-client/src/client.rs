use reqwest::{Url};

use std::{ boxed::Box };

use crate::{models::{InfluxError, Precision, TimestampOptions}, traits::PointSerialize, transporter::{InfluxDBTransporter, Transporter}};

#[derive(Debug, Clone)]
pub struct ClientOptions {
    pub host: Url,
    pub token: String,
    pub bucket: Option<String>,
    pub org: Option<String>,
    pub org_id: Option<String>,
    pub precision: Precision,
}

/// Client for InfluxDB
pub struct Client {
    options: ClientOptions,
    transporter: Box<dyn Transporter>,
}

impl Client {
    pub fn new<T>(host: T, token: T) -> Self
    where
        T: Into<String>,
    {
        let host_url = reqwest::Url::parse(&host.into()[..]).unwrap();


        Client {
            options: ClientOptions {
                host: host_url,
                token: token.into(),
                bucket: None,
                org: None,
                org_id: None,
                precision: Precision::NS,
            },
            transporter: Box::new(InfluxDBTransporter::new()),
        }
    }

    pub fn with_bucket<T: Into<String>>(mut self, bucket: T) -> Self {
        self.options.bucket = Some(bucket.into());
        self
    }

    pub fn with_org<T: Into<String>>(mut self, org: T) -> Self {
        self.options.org = Some(org.into());
        self
    }

    pub fn with_org_id<T: Into<String>>(mut self, org_id: T) -> Self {
        self.options.org_id = Some(org_id.into());
        self
    }

    pub fn with_precision(mut self, precision: Precision) -> Self {
        self.options.precision = precision;
        self
    }

    pub fn precision(&self) -> &str {
        self.options.precision.to_string()
    }

    pub fn insert_points<'b, I: IntoIterator<Item = &'b (impl PointSerialize + 'b)>>(
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
            .collect::<Vec<String>>();

        let precision = self.options.precision.to_string();
        let write_query_params = vec![("precision", precision)];

        self.transporter.insert(self.options.clone(), &body, &write_query_params)
    }
}
