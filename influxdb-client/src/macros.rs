

#[macro_export]
macro_rules! timestamp {
    ($timestamp:expr) => {
        TimestampOptions::Use(Timestamp::from($timestamp));
    };
}