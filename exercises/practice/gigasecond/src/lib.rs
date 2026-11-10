use time::{Duration, PlainDateTime as DateTime};

// 返回 start 过 10^9 秒（1 gigasecond）之后的时间
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1_000_000_000)
}
