use chrono::{DateTime, TimeDelta, Utc};

pub enum DateTimeFunction {
    Date(DateTimeArguments),
    Time(DateTimeArguments),
    DateTime(DateTimeArguments),
    JulianDay(DateTimeArguments),
    UnixEpoch(DateTimeArguments),
    StrFTime(String, DateTimeArguments),
    TimeDiff(DateTime<Utc>, DateTime<Utc>),
}

pub struct DateTimeArguments {
    pub date_time: DateTime<Utc>,
    pub modifiers: Vec<DateModifier>,
}

pub enum DateModifier {
    Year(f64),
    Month(f64),
    Day(f64),
    Hour(f64),
    Minute(f64),
    Second(f64),
    TimeDelta(TimeDelta),
    Ceiling,
    Floor,
    StartOfYear,
    StartOfMonth,
    StartOfDay,
    UnixEpoch,
    JulianDay,
    Auto,
    LocalTime,
    Utc,
    Subsecond,
    Weekday(u8),
}
