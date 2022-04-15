use influxdb_client::{Point, PointSerialize, Timestamp};

#[test]
fn test_point_serialize_with_timestamp_from_point() {
    let expected = "mem,host=host1 used_percent=23.43234543,mem_free=123456i 1556813561098000000";

    let point = Point::new("mem")
        .tag("host", "host1")
        .field("used_percent", 23.43234543)
        .field("mem_free", 123456)
        .timestamp(1556813561098000000);

    let actual = point.serialize_with_timestamp(None);

    assert_eq!(actual, expected);
}

#[test]
fn test_point_serialize_with_timestamp() {
    let expected = "mem,host=host1,origin=origin1 used_percent=23.43234543,mem_free=123456i 420";

    let point = Point::new("mem")
        .tag("host", "host1")
        .tag("origin", "origin1")
        .field("used_percent", 23.43234543)
        .field("mem_free", 123456)
        .timestamp(1556896326);

    let actual = point.serialize_with_timestamp(Some(Timestamp::from(420)));

    assert_eq!(actual, expected);
}

#[test]
fn test_point_serialize() {
    let expected = "mem,host=host1 used_percent=23.43234543,mem_free=123456i,name=\"Julius\"";

    let point = Point::new("mem")
        .tag("host", "host1")
        .field("used_percent", 23.43234543)
        .field("mem_free", 123456)
        .field("name", "Julius")
        .timestamp(1556896326);

    let actual = point.serialize();

    assert_eq!(actual, expected);
}

#[test]
fn test_point_serialize_without_tag() {
    let expected = "mem name=\"Julius\"";

    let point = Point::new("mem").field("name", "Julius");

    let actual = point.serialize();

    assert_eq!(actual, expected);
}

#[test]
fn test_point_serialize_only_measurement() {
    let expected = "mem";

    let point = Point::new("mem");

    let actual = point.serialize();

    assert_eq!(actual, expected);
}

#[test]
fn test_point_serialize_all_types() {
    let expected = r#"mem,tag_string=Hello\ world\ :D float=2.345,bool=true,int=-9223372036854775806i,field_string="Hello world :D\"" -9223372036854775806"#;

    let point = Point::new("mem")
        .tag("tag_string", "Hello world :D")
        .field("float", 2.345)
        .field("bool", true)
        .field("int", -9223372036854775806)
        .field("field_string", "Hello world :D\"")
        .timestamp(-9223372036854775806);

    let actual = point.serialize_with_timestamp(None);

    assert_eq!(actual, expected);
}

#[test]
fn test_point_serialize_emojies() {
    let expected = "mem,tagKey=🍭 fieldKey=\"Launch 🚀\" 1556813561098000000";

    let point = Point::new("mem")
        .tag("tagKey", "🍭")
        .field("fieldKey", "Launch 🚀")
        .timestamp(1556813561098000000);

    let actual = point.serialize_with_timestamp(None);

    assert_eq!(actual, expected);
}
