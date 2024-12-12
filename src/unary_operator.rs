use derive_more::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    #[display("$&&")]
    All,
    #[display("$||")]
    Any,
    #[display("$&")]
    BitAnd,
    #[display("$|")]
    BitOr,
    #[display("$+")]
    Sum,
    #[display("$*")]
    Product,
    #[display("!")]
    Not,
    #[display("-")]
    UnaryMinus,
    #[display("return")]
    Return,
    #[display("*")]
    Indirection,
    #[display("()")]
    FunctionCall,
    #[display("$]")]
    Collect,
    #[display("~")]
    Iter,
}

impl UnaryOperator {
    pub fn is_prefix(&self) -> bool {
        matches!(
            self,
            Self::Not | Self::UnaryMinus | Self::Return | Self::Indirection
        )
    }
}
