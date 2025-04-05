use crate::parser::expression::Expression;

pub enum Function {
    Nonary(NonaryFunction),
    Unary(UnaryFunction),
    Binary(BinaryFunction),
    Ternary(TernaryFunction),
    Conditional(ConditionalFunction),
    UnNary(UnNaryFunction),
}

pub struct NonaryFunction {
    pub function_name: NonaryFunctionName,
}

pub struct UnaryFunction {
    pub function_name: UnaryFunctionName,
    pub expr: Box<Expression>,
}

pub struct BinaryFunction {
    pub function_name: BinaryFunctionName,
    pub expr1: Box<Expression>,
    pub expr2: Box<Expression>,
}

pub struct TernaryFunction {
    pub function_name: TernaryFunctionName,
    pub expr1: Box<Expression>,
    pub expr2: Box<Expression>,
    pub expr3: Box<Expression>,
}

pub struct NaryFunction {
    pub function_name: NaryFunctionName,
    pub exprs: Vec<Box<Expression>>,
}

pub struct UnNaryFunction {
    pub function_name: UnNaryFunctionName,
    pub expr1: Box<Expression>,
    pub exprs: Vec<Box<Expression>>,
}

pub struct BiNaryFunction {
    pub function_name: BiNaryFunctionName,
    pub expr1: Box<Expression>,
    pub expr2: Box<Expression>,
    pub exprs: Vec<Box<Expression>>,
}

pub struct TerNaryFunction {
    pub function_name: TerNaryFunctionName,
    pub expr1: Box<Expression>,
    pub expr2: Box<Expression>,
    pub expr3: Box<Expression>,
    pub exprs: Vec<Box<Expression>>,
}

pub struct ConditionalFunction {
    pub conditional_function_name: ConditionalFunctionName,
    pub conditions: Vec<IfCondition>,
}

pub struct IfCondition {
    pub condition: Box<Expression>,
    pub value: Box<Expression>,
}

pub enum NonaryFunctionName {
    Changes,
    LastInsertRowId,
    Pi,
    Random,
    TotalChanges,
}

pub enum UnaryFunctionName {
    Abs,
    ACos,
    ACosH,
    ASin,
    ASinH,
    ATan,
    ATanH,
    BinaryNot,
    Ceiling,
    Cos,
    CosH,
    Degrees,
    Exp,
    Floor,
    Group,
    Hex,
    IsNull,
    JSON,
    JSONArrayLength,
    JSONB,
    JSONEach,
    JSONErrorPosition,
    JSONExtractByPath,
    JSONExtractByPathSQL,
    JSONPretty,
    JSONQuote,
    JSONTree,
    JSONType,
    JSONValid,
    Length,
    Likely,
    Ln,
    LoadExtension,
    Log,
    Log10,
    Log2,
    LogicalNot,
    Lower,
    LTrim,
    Negative,
    OctetLength,
    Positive,
    Radians,
    RandomBlob,
    Round,
    RTrim,
    Sign,
    Sin,
    SinH,
    SoundEx,
    SQLCompileOptionGet,
    SQLCompileOptionUsed,
    SQLiteOffset,
    SQLiteSourceId,
    SQLiteVersion,
    Sqrt,
    Tan,
    TanH,
    Trim,
    Trunc,
    TypeOf,
    UnHex,
    Unicode,
    Unlikely,
    Upper,
    ZeroBlob,
}

pub enum BinaryFunctionName {
    Add,
    ATan2,
    BinaryAnd,
    BinaryOr,
    Collate,
    Divide,
    Equals,
    Extract,
    Glob,
    GreaterThan,
    GreaterThanOrEquals,
    IfNull,
    InsStr,
    Is,
    IsDistinctFrom,
    JSONArrayLength,
    JSONBPatch,
    JSONPatch,
    JSONType,
    JSONValid,
    LessThan,
    LessThanOrEquals,
    Like,
    Liklihood,
    LoadExtension,
    Log,
    LogicalAnd,
    LogicalOr,
    LTrim,
    Mod,
    Modulus,
    Multiply,
    Power,
    Round,
    RTrim,
    ShiftLeft,
    ShiftRight,
    Substring,
    Subtract,
    Trim,
    UnHex,
}

pub enum TernaryFunctionName {
    Between,
    Like,
    Replace,
    Substring,
}

pub enum NaryFunctionName {
    Char,
    Coalesce,
    Concat,
    JSONArray,
    JSONBArray,
    JSONBGroupArray,
    JSONBGroupObject,
    JSONGroupArray,
    JSONGroupObject,
    Max,
    Min,
}

pub enum UnNaryFunctionName {
    ConcatWS,
    Format,
    PrintF,
}

pub enum BiNaryFunctionName {
    JSONBExtract,
    JSONBObject,
    JSONBRemove,
    JSONExtract,
    JSONObject,
    JSONRemove,
}

pub enum TerNaryFunctionName {
    JSONBInsert,
    JSONBReplace,
    JSONBSet,
    JSONInsert,
    JSONReplace,
    JSONSet,
}

pub enum ConditionalFunctionName {
    If,
    Iif,
    NullIf,
    Case,
}
