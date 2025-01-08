use crate::{variable::Variable, Code, Interpreter};
use lazy_static::lazy_static;

#[macro_export]
macro_rules! lazy_var {
    ( $ident: ident, $code:expr ) => {
        lazy_static! {
            pub static ref $ident: Variable = Code::parse(&Interpreter::without_stdlib(), $code)
                .unwrap()
                .exec()
                .unwrap();
        }
    };
}

lazy_var!(
    AND,
    "(iter: () -> (bool, int)) -> int {
        return iter $!0 (acc: int, curr: int) -> int {
            return acc & curr;
        }
    }"
);

lazy_var!(
    OR,
    "(iter: () -> (bool, int)) -> int {
        return iter $0 (acc: int, curr: int) -> int {
            return acc | curr;
        }
    }"
);

lazy_var!(
    ALL,
    "(iter: () -> (bool, bool)) -> bool {
        loop {
            (con, value) := iter();
            if !con { break; }
            if !value { return false; }
        }
        return true;
    }"
);

lazy_var!(
    ANY,
    "(iter: () -> (bool, bool)) -> bool {
        loop {
            (con, value) := iter();
            if !con { break; }
            if value { return true; }
        }
        return false;
    }"
);

lazy_var!(
    INT_PRODUCT,
    "(iter: () -> (bool, int)) -> int {
            return iter $1 (acc: int, curr: int) -> int {
                return acc * curr;
            }
        }"
);

lazy_var!(
    FLOAT_PRODUCT,
    "(iter: () -> (bool, float)) -> float {
        return iter $1.0 (acc: float, curr: float) -> float {
            return acc * curr;
        }
    }"
);

lazy_var!(
    INT_SUM,
    "(iter: () -> (bool, int)) -> int {
        return iter $0 (acc: int, curr: int) -> int {
            return acc + curr;
        }
    }"
);

lazy_var!(
    FLOAT_SUM,
    "(iter: () -> (bool, float)) -> float {
        return iter $0.0 (acc: float, curr: float) -> float {
            return acc + curr;
        }
    }"
);

lazy_var!(
    STRING_SUM,
    r#"(iter: () -> (bool, string)) -> string {
        return iter $"" (acc: string, curr: string) -> string {
            return acc + curr;
        }
    }"#
);

pub fn add_operators(interpreter: &mut Interpreter) {
    interpreter.insert("bitand_reduce".into(), AND.clone());
    interpreter.insert("bitor_reduce".into(), OR.clone());
    interpreter.insert("all".into(), ALL.clone());
    interpreter.insert("any".into(), ANY.clone());
    interpreter.insert("int_product".into(), INT_PRODUCT.clone());
    interpreter.insert("float_product".into(), FLOAT_PRODUCT.clone());
    interpreter.insert("int_sum".into(), INT_SUM.clone());
    interpreter.insert("float_sum".into(), FLOAT_SUM.clone());
    interpreter.insert("string_sum".into(), STRING_SUM.clone());
}
