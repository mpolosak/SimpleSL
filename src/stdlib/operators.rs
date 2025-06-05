use crate::{self as simplesl, Code, Interpreter, variable::Variable};
use lazy_static::lazy_static;
use simplesl_macros::var;

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

lazy_static! {
    pub static ref OPERATORS: Variable = {
        let and = AND.clone();
        let or = OR.clone();
        let all = ALL.clone();
        let any = ANY.clone();
        let int_product = INT_PRODUCT.clone();
        let float_product = FLOAT_PRODUCT.clone();
        let int_sum = INT_SUM.clone();
        let float_sum = FLOAT_SUM.clone();
        let string_sum = STRING_SUM.clone();
        var!(struct{
            bitand_reduce = and,
            bitor_reduce = or,
            all = all,
            any = any,
            int_product = int_product,
            float_product = float_product,
            int_sum = int_sum,
            float_sum = float_sum,
            string_sum = string_sum
        })
    };
}
