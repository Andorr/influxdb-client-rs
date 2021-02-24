pub trait HelloMacro {
    fn hello_macro();
}

pub trait PointSerialize {
    fn serialize(&self) -> String;
    fn serialize_with_timestamp(&self, timestamp: Option<String>) -> String;
}
