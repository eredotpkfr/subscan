use chrono::{DateTime, TimeDelta, Utc};
use serde::Serializer;

/// Serializer method to convert [`DateTime`] objects to strings
///
/// # Examples
///
/// ```
/// use subscan::utilities::serializers::dt_to_string_method;
/// use chrono::DateTime;
/// use serde_json::Serializer;
///
/// let mut buffer = Vec::new();
/// let mut serializer = Serializer::new(&mut buffer);
///
/// let serialized = dt_to_string_method(&DateTime::UNIX_EPOCH, &mut serializer);
/// let expected = "\"1970-01-01 00:00:00 UTC\"";
///
/// assert_eq!(String::from_utf8(buffer).unwrap(), expected);
/// ```
pub fn dt_to_string_method<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&dt.to_string())
}

/// Serializer method to convert [`TimeDelta`] objects to [`i64`] seconds
///
/// # Examples
///
/// ```
/// use subscan::utilities::serializers::td_num_seconds_method;
/// use chrono::TimeDelta;
/// use serde_json::Serializer;
///
/// let mut buffer = Vec::new();
/// let mut serializer = Serializer::new(&mut buffer);
///
/// let serialized = td_num_seconds_method(&TimeDelta::zero(), &mut serializer);
///
/// assert_eq!(String::from_utf8(buffer).unwrap(), "0");
/// ```
pub fn td_num_seconds_method<S>(td: &TimeDelta, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(td.num_seconds())
}
