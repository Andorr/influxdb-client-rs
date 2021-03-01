pub trait HelloMacro {
    fn hello_macro();
}

use crate::models::Value;

pub trait PointSerialize {
    fn serialize(&self) -> String;
    fn serialize_with_timestamp(&self, timestamp: Option<Value>) -> String;
}
