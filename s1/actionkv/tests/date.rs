#[warn(unused_imports)]
use chrono::{DateTime, FixedOffset, Local, Duration, TimeZone, NaiveDate, NaiveDateTime};


mod tests {
    use std::fmt::Debug;
    use super::*;

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
        let date = NaiveDate::from_ymd(2022, 1, 1);
        println!("Date: {}", date);
        // 解析时间 format 和 parse
        let datetime = NaiveDateTime::parse_from_str("2022-01-01 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();
        let formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("Formatted datetime: {}", formatted_datetime);
        // 日期增减
        let new_date = local + Duration::days(7);
        println!("7 date later: {}", new_date.format("%Y-%m-%d %H:%M:%S"));

        let new_date = local - Duration::days(7);
        println!("7 date ago: {}", new_date.format("%Y-%m-%d %H:%M:%S"));

        // 日期比较
        let date1 = NaiveDate::from_ymd(2022, 1, 1);
        let date2 = NaiveDate::from_ymd(2022, 1, 10);
        let ordering = date1.cmp(&date2);
        println!("Date1 is {:?} Date2", ordering);



        let date1 = NaiveDate::from_ymd(2022, 1, 1);
        let date2 = NaiveDate::from_ymd(2022, 1, 10);
        let duration = date2.signed_duration_since(date1);
        let days_diff = duration.num_days();
        println!("Days difference: {}", days_diff);
    }
}