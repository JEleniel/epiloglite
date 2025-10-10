/// Which vacuum mode to use
#[derive(Debug, Clone, PartialEq)]
pub enum VacuumMode {
    None,
    SQLiteIncremental,
    SQLiteFull,
    EpilogLite,
}
