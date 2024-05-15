pub mod mongo;

use chrono::{DateTime, FixedOffset, Local, Utc};

pub fn chrono_utc_to_local(f: &DateTime<Utc>) -> DateTime<FixedOffset> {
    let fixed_offset = FixedOffset::east_opt(8 * 3600).unwrap(); // 转为 utc+8 东八区
    let local_date_time = f.with_timezone(&fixed_offset);
    local_date_time
}

pub fn format_chrono_utc_to_local(f: &DateTime<Utc>) -> String {
    let fixed_offset = FixedOffset::east_opt(8 * 3600).unwrap(); // 转为 utc+8 东八区
    let local_date_time = f.with_timezone(&fixed_offset);
    local_date_time.format("%Y-%m-%d %H:%M:%S%.3f %z").to_string()
}
