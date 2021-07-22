use std::fmt::Write;

use crate::traits::PointSerialize;

use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl TryFrom<Value> for String {
    type Error = &'static str;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Str(string) = value {
            return Ok(string);
        } else {
            return Err("Could not convert from value to String");
        }
    }
}
impl TryFrom<Value> for i64 {
    type Error = &'static str;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Int(int) = value {
            return Ok(int);
        } else {
            return Err("Could not convert from value to i64");
        }
    }
}
impl TryFrom<Value> for f64 {
    type Error = &'static str;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Float(float) = value {
            return Ok(float);
        } else {
            return Err("Could not convert from value to f64");
        }
    }
}
impl TryFrom<Value> for bool {
    type Error = &'static str;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Bool(bool) = value {
            return Ok(bool);
        } else {
            return Err("Could not convert from value to bool");
        }
    }
}

impl TryFrom<Value> for Timestamp {
    type Error = &'static str;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Str(string) => Ok(string.as_str().into()),
            Value::Int(int) => Ok(int.into()),
            _ => Err("Could not create timestamp from field"),
        }
    }
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

    fn check_if_can_deserialize<T>(
        fields: &std::collections::HashMap<String, T>,
    ) -> Result<(), String> {
        todo!()
    }

    fn deserialize_from_hashmap(fields: &std::collections::HashMap<String, Value>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum InfluxError {
    InvalidSyntax(String),
    InvalidCredentials(String),
    Forbidden(String),
    Unknown(String),
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
