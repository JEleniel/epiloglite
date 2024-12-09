use logos::Logos;
use regex::Regex;

#[derive(Debug, PartialEq, Clone, Default)]
enum LexingError {
    UnusedParameter(String),
    ParserSyntaxError(String, String),
    UpdateDeleteLimitError(String, String, String),
    StackOverflow(String),
    #[default]
    Other,
}

pub struct Keyword {
    pub name: String,
    pub token_type: String,
    pub mask: u16,
    pub priority: u16,
    pub id: u16,
    pub hash: u16,
    pub offset: u16,
    pub length: u16,
    pub prefix: u16,
    pub longest_suffix: u16,
    pub next_index: u16,
    pub substring_id: u16,
    pub substring_offset: u16,
    pub original_name: String,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error=LexingError)]
enum Token {
    // Keywords
    #[token("abort", ignore(case))]
    Abort,
    #[token("action", ignore(case))]
    Action,
    #[token("add", ignore(case))]
    Add,
    #[token("after", ignore(case))]
    After,
    #[token("all", ignore(case))]
    All,
    #[token("alter", ignore(case))]
    Alter,
    #[token("always", ignore(case))]
    Always,
    #[token("analyze", ignore(case))]
    Analyze,
    #[token("and", ignore(case))]
    And,
    #[token("as", ignore(case))]
    As,
    #[token("asc", ignore(case))]
    Ascending,
    #[token("attach", ignore(case))]
    Attach,
    #[token("autoincrement", ignore(case))]
    AutoIncrement,
    #[token("before", ignore(case))]
    Before,
    #[token("begin", ignore(case))]
    Begin,
    #[token("between", ignore(case))]
    Between,
    #[token("by", ignore(case))]
    By,
    #[token("cascade", ignore(case))]
    Cascade,
    #[token("case", ignore(case))]
    Case,
    #[token("cast", ignore(case))]
    Cast,
    #[token("check", ignore(case))]
    Check,
    #[token("collate", ignore(case))]
    Collate,
    #[token("column", ignore(case))]
    Column,
    #[token("commit", ignore(case))]
    Commit,
    #[token("constraint", ignore(case))]
    Constraint,
    #[token("create", ignore(case))]
    Create,
    #[token("cross", ignore(case))]
    Cross,
    #[token("current", ignore(case))]
    Current,
    #[token("current_date", ignore(case))]
    CurrentDate,
    #[token("current_time", ignore(case))]
    CurrentTime,
    #[token("current_timestamp", ignore(case))]
    CurrentTimestamp,
    #[token("database", ignore(case))]
    Database,
    #[token("default", ignore(case))]
    Default,
    #[token("deferred", ignore(case))]
    Deferred,
    #[token("deferrable", ignore(case))]
    Deferrable,
    #[token("delete", ignore(case))]
    Delete,
    #[token("desc", ignore(case))]
    Descending,
    #[token("detach", ignore(case))]
    Detach,
    #[token("distinct", ignore(case))]
    Distinct,
    #[token("do", ignore(case))]
    Do,
    #[token("drop", ignore(case))]
    Drop,
    #[token("end", ignore(case))]
    End,
    #[token("each", ignore(case))]
    Each,
    #[token("else", ignore(case))]
    Else,
    #[token("escape", ignore(case))]
    Escape,
    #[token("except", ignore(case))]
    Except,
    #[token("exclusive", ignore(case))]
    Exclusive,
    #[token("exclude", ignore(case))]
    Exclude,
    #[token("exists", ignore(case))]
    Exists,
    #[token("explain", ignore(case))]
    Explain,
    #[token("fail", ignore(case))]
    Fail,
    #[token("filter", ignore(case))]
    Filter,
    #[token("first", ignore(case))]
    First,
    #[token("following", ignore(case))]
    Following,
    #[token("for", ignore(case))]
    For,
    #[token("foreign", ignore(case))]
    Foreign,
    #[token("from", ignore(case))]
    From,
    #[token("full", ignore(case))]
    Full,
    #[token("generated", ignore(case))]
    Generated,
    #[token("glob", ignore(case))]
    Glob,
    #[token("group", ignore(case))]
    Group,
    #[token("groups", ignore(case))]
    Groups,
    #[token("having", ignore(case))]
    Having,
    #[token("if", ignore(case))]
    If,
    #[token("ignore", ignore(case))]
    Ignore,
    #[token("immediate", ignore(case))]
    Immediate,
    #[token("in", ignore(case))]
    In,
    #[token("index", ignore(case))]
    Index,
    #[token("indexed", ignore(case))]
    Indexed,
    #[token("initially", ignore(case))]
    Initially,
    #[token("inner", ignore(case))]
    Inner,
    #[token("insert", ignore(case))]
    Insert,
    #[token("instead", ignore(case))]
    Instead,
    #[token("intersect", ignore(case))]
    Intersect,
    #[token("into", ignore(case))]
    Into,
    #[token("is", ignore(case))]
    Is,
    #[token("isnull", ignore(case))]
    IsNull,
    #[token("join", ignore(case))]
    Join,
    #[token("key", ignore(case))]
    Key,
    #[token("last", ignore(case))]
    Last,
    #[token("left", ignore(case))]
    Left,
    #[token("like", ignore(case))]
    Like,
    #[token("limit", ignore(case))]
    Limit,
    #[token("match", ignore(case))]
    Match,
    #[token("materialized", ignore(case))]
    Materialized,
    #[token("natural", ignore(case))]
    Natural,
    #[token("no", ignore(case))]
    No,
    #[token("not", ignore(case))]
    Not,
    #[token("nothing", ignore(case))]
    Nothing,
    #[token("notnull", ignore(case))]
    NotNull,
    #[token("null", ignore(case))]
    Null,
    #[token("nulls", ignore(case))]
    Nulls,
    #[token("of", ignore(case))]
    Of,
    #[token("offset", ignore(case))]
    Offset,
    #[token("on", ignore(case))]
    On,
    #[token("or", ignore(case))]
    Or,
    #[token("order", ignore(case))]
    Order,
    #[token("others", ignore(case))]
    Others,
    #[token("outer", ignore(case))]
    Outer,
    #[token("over", ignore(case))]
    Over,
    #[token("partition", ignore(case))]
    Partition,
    #[token("plan", ignore(case))]
    Plan,
    #[token("pragma", ignore(case))]
    Pragma,
    #[token("preceeding", ignore(case))]
    Preceding,
    #[token("primary", ignore(case))]
    Primary,
    #[token("query", ignore(case))]
    Query,
    #[token("raise", ignore(case))]
    Raise,
    #[token("range", ignore(case))]
    Range,
    #[token("recursive", ignore(case))]
    Recursive,
    #[token("regexp", ignore(case))]
    RegularExpression,
    #[token("reindex", ignore(case))]
    Reindex,
    #[token("release", ignore(case))]
    Release,
    #[token("rename", ignore(case))]
    Rename,
    #[token("replace", ignore(case))]
    Replace,
    #[token("restrict", ignore(case))]
    Restrict,
    #[token("returning", ignore(case))]
    Returning,
    #[token("right", ignore(case))]
    Right,
    #[token("rollback", ignore(case))]
    Rollback,
    #[token("row", ignore(case))]
    Row,
    #[token("rows", ignore(case))]
    Rows,
    #[token("savepoint", ignore(case))]
    SavePoint,
    #[token("select", ignore(case))]
    Select,
    #[token("set", ignore(case))]
    Set,
    #[token("table", ignore(case))]
    Table,
    #[token("temp", ignore(case))]
    Temp,
    #[token("temporary", ignore(case))]
    Temporary,
    #[token("then", ignore(case))]
    Then,
    #[token("ties", ignore(case))]
    Ties,
    #[token("to", ignore(case))]
    To,
    #[token("transaction", ignore(case))]
    Transaction,
    #[token("trigger", ignore(case))]
    Trigger,
    #[token("unbounded", ignore(case))]
    Unbounded,
    #[token("union", ignore(case))]
    Union,
    #[token("unique", ignore(case))]
    Unique,
    #[token("update", ignore(case))]
    Update,
    #[token("using", ignore(case))]
    Using,
    #[token("vacuum", ignore(case))]
    Vacuum,
    #[token("values", ignore(case))]
    Values,
    #[token("view", ignore(case))]
    View,
    #[token("virtual", ignore(case))]
    Virtual,
    #[token("when", ignore(case))]
    When,
    #[token("where", ignore(case))]
    Where,
    #[token("window", ignore(case))]
    Window,
    #[token("with", ignore(case))]
    With,
    #[token("within", ignore(case))]
    Within,
    #[token("without", ignore(case))]
    Without,

    // Literals
    // Time with delimeters
    #[regex(r"'(?:0 |[12][0-9] |3[0-4] )?(?:[01]?[0-9]|2[0-3]):(?:[0-5]?[0-9]):(?:[0-5]?[0-9])(?:\.[0-9]{1,6})?(?:\+(?:(?:0[0-9]|1[0-3]):[0-5][0-9]|14:00)|\-(?:0[1-9]|[1-3][0-9]:[0-5][0-9]))?'")]
    // Time without delimeters
    #[regex(r"'(?:[01][0-9]|2[0-3])(?:[0-5][0-9])(?:[0-5][0-9])(?:\.[0-9]{1,6})?(?:\+(?:(?:0[0-9]|1[0-3]):[0-5][0-9]|14:00)|\-(?:0[1-9]|[1-3][0-9]:[0-5][0-9]))?'")]
    Time,
    #[token("TRUE", ignore(case))]
    #[token("FALSE", ignore(case))]
    BooleanLiteral,

    // Complex symbolic operators
    #[token("??(")]
    LeftParenthesisTrigraph,
    #[token("??)")]
    RightParentehesisTrigraph,
    #[token("<>")]
    InequalityOperator,
    #[token(">=")]
    GreaterThanOrEqualOperator,
    #[token("<=")]
    LessThanOrEqualOperator,
    #[token("||")]
    ConcatenationOperator,
    #[token("->")]
    RightArrow,
    #[token("::")]
    DoubleColon,
    #[token("..")]
    DoubleFullStop,

    // Individual characters
    #[token(" ")]
    Space,
    #[token("\x22")]
    QuotationMark,
    #[token("%")]
    PercentSign,
    #[token("&")]
    Ampersand,
    #[token("'")]
    Apostrophe,
    #[token("(")]
    LeftParenthesis,
    #[token(")")]
    RightParenthesis,
    #[token("*")]
    Asterisk,
    #[token("+")]
    PlusSign,
    #[token(",")]
    Comma,
    #[token("-")]
    HyphenMinus,
    #[token(".")]
    FullStop,
    #[token("/")]
    Solidus,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("=")]
    EqualsSign,
    #[token("<")]
    LessThanSign,
    #[token(">")]
    GreaterThanSign,
    #[token("?")]
    QuestionMark,
    #[token("[")]
    LeftSquareBracket,
    #[token("]")]
    RightSquareBracket,
    #[token("^")]
    CircumflexAccent,
    #[token("_")]
    LowLine,
    #[token("|")]
    VerticalLine,
    #[token("{")]
    LeftCurlyBracket,
    #[token("}")]
    RightCurlyBracket,
}
