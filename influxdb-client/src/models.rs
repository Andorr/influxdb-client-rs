use std::fmt::Write;

use crate::traits::PointSerialize;

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
        write!(&mut builder, "{}", self.measurement).unwrap();

        // Write tags
        if !self.tags.is_empty() {
            write!(&mut builder, ",").unwrap();

            let tags = self
                .tags
                .iter()
                .map(|field| format!("{}={}", field.0, field.1.to_string()))
                .collect::<Vec<String>>()
                .join(",")
                .clone();

            builder.push_str(&tags);
        }

        // Write fields
        if !self.fields.is_empty() {
            write!(&mut builder, " ").unwrap();

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
                .join(",")
                .clone();

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
                    .unwrap_or(Timestamp::from(0))
                    .to_string()
            ),
        }
    }
}

#[derive(Debug)]
pub enum InfluxError {
    InvalidSyntax(String),
    InvalidCredentials(String),
    Forbidden(String),
    Unknown(String),
}

#[cfg(test)]
mod tests {
    use super::{Point, PointSerialize, Timestamp};

    #[test]
    fn test_point_serialize_with_timestamp_from_point() {
        let expected = "mem,host=host1 used_percent=23.43234543 1556896326";

        let point = Point::new("mem")
            .tag("host", "host1")
            .field("used_percent", 23.43234543)
            .timestamp(1556896326);

        let actual = point.serialize_with_timestamp(None);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_serialize_with_timestamp() {
        let expected = "mem,host=host1,origin=origin1 used_percent=23.43234543 420";

        let point = Point::new("mem")
            .tag("host", "host1")
            .tag("origin", "origin1")
            .field("used_percent", 23.43234543)
            .timestamp(1556896326);

        let actual = point.serialize_with_timestamp(Some(Timestamp::from(420)));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_serialize() {
        let expected = "mem,host=host1 used_percent=23.43234543,name=\"Julius\"";

        let point = Point::new("mem")
            .tag("host", "host1")
            .field("used_percent", 23.43234543)
            .field("name", "Julius")
            .timestamp(1556896326);

        let actual = point.serialize();

        assert_eq!(actual, expected);
    }
}
