#![cfg_attr(
    debug_assertions,
    allow(
        unused,
        dead_code,
        unused_imports,
        unused_variables,
        unused_assignments,
        non_snake_case
    )
)]
use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StructWithCustomDate {
    // DateTime 可以直接支持 Serde，但使用 RFC3339 格式。提供一些自定义逻辑使其使用我们想要的格式。
    #[serde(with = "my_date_format")]
    pub timestamp: DateTime<Utc>,

    // 结构体中的其他字段。
    pub bidder: String,
}

mod my_date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // serialize_with 函数的签名必须遵循以下模式：
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // 尽管也可以对输入类型 T 进行泛型化。
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
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
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

mod tests {
    use super::*;
    use std::fmt::Debug;
    #[test]
    fn test_date_format() {
        let json_str = r#"
        {
            "timestamp": "2017-02-16 21:54:30",
            "bidder": "Skrillex"
        }
        "#;

        let data: StructWithCustomDate = serde_json::from_str(json_str).unwrap();
        println!("{:#?}", data);

        let serialized = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", serialized);
    }
    #[test]
    fn test_date() {
        let local: DateTime<Local> = Local::now();
        println!("{:?}", local);

        let china = FixedOffset::east_opt(8 * 3600).unwrap();
        println!("{:?}", local.with_timezone(&china));
        //
        let f = local.format("%Y-%m-%d %H:%M:%S");
        println!("formatted= {:?}", f.to_string());
        println!("timestamp seconds={:?}", local.timestamp());
        println!("native time={:?}", local.time());
        let date = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
        println!("Date: {}", date);
        // 解析时间 format 和 parse
        let datetime =
            NaiveDateTime::parse_from_str("2022-01-01 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
        let formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("Formatted datetime: {}", formatted_datetime);
        // 日期增减
        let new_date = local + Duration::days(7);
        println!("7 date later: {}", new_date.format("%Y-%m-%d %H:%M:%S"));

        let new_date = local - Duration::days(7);
        println!("7 date ago: {}", new_date.format("%Y-%m-%d %H:%M:%S"));

        // 日期比较
        let date1 = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2022, 1, 10).unwrap();
        let ordering = date1.cmp(&date2);
        println!("Date1 is {:?} Date2", ordering);

        let date1 = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2022, 1, 10).unwrap();
        let duration = date2.signed_duration_since(date1);
        let days_diff = duration.num_days();
        println!("Days difference: {}", days_diff);
    }
}
