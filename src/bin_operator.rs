use derive_more::Display;
use simplesl_parser::Rule;

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

#[doc(hidden)]
impl From<Rule> for BinOperator {
    fn from(value: Rule) -> Self {
        match value {
            Rule::equal => Self::Equal,
            Rule::not_equal => Self::NotEqual,
            Rule::lshift => Self::LShift,
            Rule::rshift => Self::RShift,
            Rule::greater => Self::Greater,
            Rule::greater_equal => Self::GreaterOrEqual,
            Rule::lower => Self::Lower,
            Rule::lower_equal => Self::LowerOrEqual,
            Rule::and => Self::And,
            Rule::or => Self::Or,
            Rule::bitwise_and => Self::BitwiseAnd,
            Rule::bitwise_or => Self::BitwiseOr,
            Rule::xor => Self::Xor,
            Rule::pow => Self::Pow,
            Rule::multiply => Self::Multiply,
            Rule::divide => Self::Divide,
            Rule::add => Self::Add,
            Rule::subtract => Self::Subtract,
            Rule::modulo => Self::Modulo,
            Rule::map => Self::Map,
            Rule::filter => Self::Filter,
            Rule::assign => Self::Assign,
            Rule::assign_add => Self::AssignAdd,
            Rule::assign_subtract => Self::AssignSubtract,
            Rule::assing_multiply => Self::AssignMultiply,
            Rule::assign_divide => Self::AssignDivide,
            Rule::assign_modulo => Self::AssignModulo,
            Rule::assign_lshift => Self::AssignLShift,
            Rule::assign_rshift => Self::AssignRShift,
            Rule::assign_bitwise_and => Self::AssignBitwiseAnd,
            Rule::assign_bitwise_or => Self::AssignBitwiseOr,
            Rule::assign_xor => Self::AssignXor,
            Rule::assign_pow => Self::AssignPow,
            _ => unreachable!(),
        }
    }
}
