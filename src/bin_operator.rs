use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum BinOperator {
    #[display("+")]
    Add,
    #[display("-")]
    Subtract,
    #[display("*")]
    Multiply,
    #[display("/")]
    Divide,
    #[display("%")]
    Modulo,
    #[display("**")]
    Pow,
    #[display("==")]
    Equal,
    #[display("!=")]
    NotEqual,
    #[display(">")]
    Greater,
    #[display(">=")]
    GreaterOrEqual,
    #[display("<")]
    Lower,
    #[display("<=")]
    LowerOrEqual,
    #[display("&&")]
    And,
    #[display("||")]
    Or,
    #[display("&")]
    BitwiseAnd,
    #[display("|")]
    BitwiseOr,
    #[display("^")]
    Xor,
    #[display("<<")]
    LShift,
    #[display(">>")]
    RShift,
    #[display("?")]
    Filter,
    #[display("@")]
    Map,
    At,
    FunctionCall,
    #[display("=")]
    Assign,
    #[display("+=")]
    AssignAdd,
    #[display("-=")]
    AssignSubtract,
    #[display("*=")]
    AssignMultiply,
    #[display("/=")]
    AssignDivide,
    #[display("%=")]
    AssignModulo,
    #[display("<<=")]
    AssignLShift,
    #[display(">>=")]
    AssignRShift,
    #[display("&=")]
    AssignBitwiseAnd,
    #[display("|=")]
    AssignBitwiseOr,
    #[display("^=")]
    AssignXor,
    #[display("**=")]
    AssignPow,
}
