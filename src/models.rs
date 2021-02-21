use std::fmt::Write;

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

impl Value {
    fn to_string(self) -> String {
        match self {
            Value::Str(s) => s.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
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
                write!(&mut builder, "{}={}", tag.0.to_string(), tag.1.to_string()).unwrap();
            }
        }

        // Write fields
        if !self.fields.is_empty() {
            write!(&mut builder, " ").unwrap();

            for field in self.fields {
                write!(
                    &mut builder,
                    "{}={}",
                    field.0.to_string(),
                    field.1.to_string()
                )
                .unwrap();
            }
        }

        // Write timestamp
        if let Some(t) = self.timestamp {
            write!(&mut builder, " {}", t).unwrap();
        }

        builder
    }
}

impl Iterator for Point {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Debug)]
pub enum InfluxError {
    InvalidSyntax(String),
    InvalidCredentials(String),
    Forbidden(String),
    Unknown(String),
}

mod tests {
    use super::Point;

    #[test]
    fn test_point_serialize() {
        let expected = "mem,host=host1 used_percent=23.43234543 1556896326";

        let point = Point::new("mem")
            .tag("host", "host1")
            .field("used_percent", 23.43234543)
            .timestamp(1556896326);

        let actual = point.serialize();

        assert_eq!(actual, expected);
    }
}
