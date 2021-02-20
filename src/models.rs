use std::fmt::Write;

#[derive(Debug)]
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

pub struct Point {

    pub measurement: String,
    pub timestamp: Option<i64>,
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

    pub fn timestamp<T: Into<i64>>(mut self, timestamp: T) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    pub fn serialize(self) -> String {
        let mut builder = String::new();

        // Write measurement
        write!(&mut builder, "{}", self.measurement).unwrap();

        // Write tags
        if !self.tags.is_empty() {
            write!(&mut builder, ",").unwrap();
            
            for tag in self.tags {
                write!(&mut builder, "{:?}={:?}", tag.0, tag.1).unwrap();
            }
        }
        
        // Write fields
        if !self.fields.is_empty() {
            write!(&mut builder, " ").unwrap();

            for field in self.fields {
                write!(&mut builder, "{:?}={:?}", field.0, field.1).unwrap();
            }
        }

        // Write timestamp
        if let Some(t) = self.timestamp {
            write!(&mut builder, " {}", t).unwrap();
        }

        builder
    }
}