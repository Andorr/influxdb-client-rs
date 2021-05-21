use crate::traits::PointSerialize;

use reqwest::Response;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl From<&str> for Value {
    fn from(v: &str) -> Value {
        Value::Str(v.to_string())
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Value {
        Value::Float(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Value {
        Value::Int(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Value {
        Value::Bool(v)
    }
}

impl std::string::ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Str(s) => s.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Timestamp {
    Str(String),
    Int(i64),
}

impl From<&str> for Timestamp {
    fn from(v: &str) -> Timestamp {
        Timestamp::Str(v.to_string())
    }
}

impl From<i64> for Timestamp {
    fn from(v: i64) -> Timestamp {
        Timestamp::Int(v)
    }
}

impl std::string::ToString for Timestamp {
    fn to_string(&self) -> String {
        match self {
            Timestamp::Str(s) => s.to_string(),
            Timestamp::Int(i) => i.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub measurement: String,
    pub timestamp: Option<Timestamp>,
    pub tags: Vec<(String, Value)>,
    pub fields: Vec<(String, Value)>,
}

impl Point {
    pub fn new<T: Into<String>>(measurement: T) -> Self {
        Point {
            measurement: measurement.into(),
            tags: Vec::new(),
            fields: Vec::new(),
            timestamp: None,
        }
    }

    pub fn tag<T: Into<String>, V: Into<Value>>(mut self, key: T, value: V) -> Self {
        self.tags.push((key.into(), value.into()));
        self
    }

    pub fn field<T: Into<String>, V: Into<Value>>(mut self, key: T, value: V) -> Self {
        self.fields.push((key.into(), value.into()));
        self
    }

    pub fn timestamp<T: Into<Timestamp>>(mut self, timestamp: T) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
}

impl PointSerialize for Point {
    fn serialize(&self) -> String {
        let mut builder = String::new();

        // Write measurement
        builder.push_str(&self.measurement);

        // Write tags
        if !self.tags.is_empty() {
            builder.push(',');

            // TODO: iterate can avoid string allocation and bring better performance
            let tags = self
                .tags
                .iter()
                .map(|field| format!("{}={}", field.0, field.1.to_string()))
                .collect::<Vec<String>>()
                .join(",");

            builder.push_str(&tags);
        }

        // Write fields
        if !self.fields.is_empty() {
            builder.push(' ');

            let fields = self
                .fields
                .iter()
                .map(|field| {
                    format!(
                        "{}={}",
                        field.0,
                        match field.1.clone() {
                            Value::Str(s) => format!("\"{}\"", s),
                            _ => field.1.to_string(),
                        }
                    )
                })
                .collect::<Vec<String>>()
                .join(",");

            builder.push_str(&fields);
        }

        builder
    }

    fn serialize_with_timestamp(&self, timestamp: Option<Timestamp>) -> String {
        match timestamp {
            Some(timestamp) => format!("{} {}", self.serialize(), timestamp.to_string()),
            None => format!(
                "{} {}",
                self.serialize(),
                self.timestamp
                    .clone()
                    .unwrap_or_else(|| Timestamp::from(0))
                    .to_string()
            ),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InfluxError {
    #[error("Network error: {0:?}")]
    Network(#[from] reqwest::Error),
    #[error("Invalid syntax: {0:?}")]
    InvalidSyntax(Response),
    #[error("Invalid credentials: {0:?}")]
    InvalidCredentials(Response),
    #[error("Forbidden: {0:?}")]
    Forbidden(Response),
    #[error("Unknown error: {0:?}")]
    Unknown(Response),
}

#[derive(Clone)]
pub enum TimestampOptions {
    None,
    Use(Timestamp),
    FromPoint,
}

pub enum Precision {
    NS,
    US,
    MS,
    S,
}

impl Precision {
    pub fn to_string(&self) -> &str {
        match self {
            Precision::NS => "ns",
            Precision::US => "us",
            Precision::MS => "ms",
            Precision::S => "s",
        }
    }
}
