use crate::{models::Timestamp, Value};

pub trait PointSerialize {
    fn serialize(&self) -> String;
    fn serialize_with_timestamp(&self, timestamp: Option<Timestamp>) -> String;
    fn check_if_can_deserialize<T>(
        fields: &std::collections::HashMap<String, T>,
    ) -> Result<(), String>;
    fn deserialize_from_hashmap(fields: &std::collections::HashMap<String, Value>) -> Self;
}
