//! Escape strings if needed.
//!
//! refer: <https://archive.docs.influxdata.com/influxdb/v1.2/write_protocols/line_protocol_tutorial/>
use std::borrow::Cow;

macro_rules! escape {
    ($s:expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {{
        let need_escape = $s.chars().any(|c| match c {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false,
        });
        match need_escape {
            true => Cow::from({
                // give an extra 10 byte space to try to avoid second memory allocation
                let mut ans = String::with_capacity($s.len() + 10);
                for ch in $s.chars() {
                    match ch {
                        $( $pattern )|+ $( if $guard )? => ans.push('\\'),
                        _ => {}
                    }
                    ans.push(ch);
                }
                ans
            }),
            false => Cow::from($s),
        }
    }};
}

/// Make an escaped string for tag and field keys
///
/// For tag keys, tag values, and field keys always use a backslash character \ to escape
pub fn escape_tag_and_field_keys(s: &str) -> Cow<str> {
    escape!(s, ',' | '=' | ' ')
}

/// Make an escaped string for tag and field keys
///
/// For measurements always use a backslash character \ to escape
pub fn escape_measurement(s: &str) -> Cow<str> {
    escape!(s, ',' | ' ')
}

/// Make an escaped string for field value
///
/// The returned string is *not* quoted in double quotes.
pub fn escape_field_value_string(s: &str) -> Cow<str> {
    escape!(s, '"')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_tag_and_field_keys() {
        assert_eq!(escape_tag_and_field_keys("us,midwest"), r"us\,midwest");
        assert_eq!(escape_tag_and_field_keys("temp=rature"), r"temp\=rature");
        assert_eq!(
            escape_tag_and_field_keys("location place"),
            r"location\ place"
        );
    }

    #[test]
    fn test_measurement_escape() {
        assert_eq!(escape_measurement("wea,ther"), r"wea\,ther");
        assert_eq!(escape_measurement("wea ther"), r"wea\ ther");
    }

    #[test]
    fn test_string_escape() {
        assert_eq!(escape_field_value_string("too\"hot"), "too\\\"hot");
    }
}
