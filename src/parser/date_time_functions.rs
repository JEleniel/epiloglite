use chrono::TimeDelta;

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
