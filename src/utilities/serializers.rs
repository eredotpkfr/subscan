use chrono::TimeDelta;
use serde::Serializer;

/// Serializer method to convert [`TimeDelta`] objects to [`i64`] seconds
///
/// # Examples
///
/// ```
/// use subscan::utilities::serializers::td_to_seconds;
/// use chrono::TimeDelta;
/// use serde_json::Serializer;
///
/// let mut buffer = Vec::new();
/// let mut serializer = Serializer::new(&mut buffer);
///
/// let serialized = td_to_seconds(&TimeDelta::zero(), &mut serializer);
///
/// assert_eq!(String::from_utf8(buffer).unwrap(), "0");
/// ```
pub fn td_to_seconds<S>(td: &TimeDelta, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(td.num_seconds())
}
