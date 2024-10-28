use chrono::{DateTime, TimeDelta, Utc};
use serde::Serializer;

pub fn dt_to_string_method<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&dt.to_string())
}

pub fn td_num_seconds_method<S>(td: &TimeDelta, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(td.num_seconds())
}
