use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime, Utc};
use mongodb::bson::{self, oid::ObjectId};
use mongodb::{
    // options, //modify here
    Client,
    Collection,
};
use serde::{
    de::{self, Deserialize},
    ser, Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::result::Result;
use time::macros::offset;
use time::{self, macros::format_description};
use tokio::sync::OnceCell;
use tracing::info;

pub fn serialize_object_id_option_as_hex_string<S: Serializer>(
    val: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match val {
        Some(oid) => oid.to_hex().serialize(serializer),
        None => serializer.serialize_none(),
    }
}

// const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

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
        info!("deserialize string to datetime {:?}", iso);
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
        info!("serialize datetime to string {:?}", formatted);
        serializer.serialize_str(&formatted)
    }
}

pub fn try_to_local_time_string(d: &bson::DateTime) -> Result<String, InteralError> {
    let dt = d.to_time_0_3();
    let local = dt.to_offset(offset!(+8));
    let f = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    local.format(f).map_err(|e| InteralError(e.to_string()))
}

pub fn parse_local_time_string(s: impl AsRef<str>) -> Result<bson::DateTime, InteralError> {
    let f = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let odt =
        time::OffsetDateTime::parse(s.as_ref(), f).map_err(|e| InteralError(e.to_string()))?;
    Ok(bson::DateTime::from_time_0_3(odt))
}

#[derive(Debug)]
pub struct InteralError(String);

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

pub mod local_date_format {
    use super::*;
    use chrono::{DateTime, Local, NaiveDateTime};
    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // serialize_with 函数的签名必须遵循以下模式：
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // 尽管也可以对输入类型 T 进行泛型化。
    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // deserialize_with 函数的签名必须遵循以下模式：
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // 尽管也可以对输出类型 T 进行泛型化。
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Local>::from_naive_utc_and_offset(
            dt,
            FixedOffset::east_opt(5 * 3600).unwrap(),
        ))
    }
}

// 支持返回值的泛化，每个子模块引用这个模块
pub fn get_collection<T>(client: &OnceCell<Client>, db: &str, collection: &str) -> Collection<T> {
    let col = client.get().unwrap().database(db).collection(collection);
    col
}
