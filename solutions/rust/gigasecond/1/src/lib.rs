use time::{Duration, PrimitiveDateTime as DateTime};

pub fn after(start: DateTime) -> DateTime {
    // todo!("What is Date Time 1 gigasecond after this{start}  ");
    return start + Duration::seconds(1_000_000_000);
}
