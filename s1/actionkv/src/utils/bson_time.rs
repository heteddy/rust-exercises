use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use mongodb::bson;
use mongodb::error::Error as MongoError;
use serde::{
    de::{self, Deserialize},
    ser, Deserializer, Serializer,
};
use std::fmt::{self, Display};
use std::result::Result;
use time::macros::{datetime, offset};
use time::OffsetDateTime;
use time::{self, macros::format_description};

use crate::pb;

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub mod bson_datetime_as_string {
    // 需要 use super::*; 才能用到外面的use;
    // 或者需要把外面的use放到这里
    use super::*;
    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    /// Deserializes a [`crate::DateTime`] from an RFC 3339 formatted string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<bson::DateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let iso = String::deserialize(deserializer)?;
        let date = NaiveDateTime::parse_from_str(&iso, FORMAT).map_err(|e| {
            de::Error::custom(format!(
                "{:?}cannot parse self defined datetime from \"{}\"",
                e, iso
            ))
        })?;
        let utc_date = date - Duration::hours(8);
        let dt = DateTime::<Utc>::from_naive_utc_and_offset(utc_date, Utc);
        Ok(dt.into())
    }

    /// Serializes a [`crate::DateTime`] as an RFC 3339 (ISO 8601) formatted string.
    pub fn serialize<S: Serializer>(
        val: &bson::DateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let formatted = try_to_local_time_string(val).map_err(|e| {
            ser::Error::custom(format!("cannot format {} as local time: {:?}", val, e))
        })?;
        serializer.serialize_str(&formatted)
    }
}

fn try_to_local_time_string(d: &bson::DateTime) -> Result<String, InteralError> {
    let dt = d.to_time_0_3();
    let local = dt.to_offset(offset!(+8));
    let f = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    local.format(f).map_err(|e| InteralError(e.to_string()))
}

fn parse_local_time_string(s: impl AsRef<str>) -> Result<bson::DateTime, InteralError> {
    let f = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let odt =
        time::OffsetDateTime::parse(s.as_ref(), f).map_err(|e| InteralError(e.to_string()))?;
    Ok(bson::DateTime::from_time_0_3(odt))
}

#[derive(Debug)]
pub struct InteralError(String);

// impl IntoResponse for InteralError {
//     fn into_response(self) -> Response {
//         self.0.into_response()
//     }
// }

impl fmt::Display for InteralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl std::error::Error for InteralError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // 泛型错误，没有记录其内部原因。
        None
    }
}
