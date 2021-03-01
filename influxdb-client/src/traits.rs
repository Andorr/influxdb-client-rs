use crate::models::Timestamp;


pub trait PointSerialize {
    fn serialize(&self) -> String;
    fn serialize_with_timestamp(&self, timestamp: Option<Timestamp>) -> String;
}