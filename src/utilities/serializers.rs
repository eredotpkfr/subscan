use chrono::{DateTime, TimeDelta, Utc};
use serde::Serializer;

/// Serializer method to convert [`DateTime`] objects to strings
///
/// # Examples
///
/// ```
/// use serde_json::json;
/// use chrono::DateTime;
///
/// let json = json!({
///     "time": DateTime::UNIX_EPOCH.to_string(),
/// });
/// let expected = "{\"time\":\"1970-01-01 00:00:00 UTC\"}";
///
/// assert_eq!(serde_json::to_string(&json).unwrap(), expected);
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
/// use serde_json::json;
/// use chrono::TimeDelta;
///
/// let json = json!({
///     "seconds": TimeDelta::zero().num_seconds(),
/// });
/// let expected = "{\"seconds\":0}";
///
/// assert_eq!(serde_json::to_string(&json).unwrap(), expected);
/// ```
pub fn td_num_seconds_method<S>(td: &TimeDelta, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(td.num_seconds())
}
