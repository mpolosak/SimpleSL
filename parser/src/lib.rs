use pest::pratt_parser::PrattParser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "simplesl.pest"]
pub struct SimpleSLParser;

lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::{Left, Right}, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(assign, Right) | Op::infix(assign_add, Right)
                | Op::infix(assign_subtract, Right) | Op::infix(assing_multiply, Right)
                | Op::infix(assign_divide, Right) | Op::infix(assign_modulo, Right)
                | Op::infix(assign_lshift, Right) | Op::infix(assign_rshift, Right)
                | Op::infix(assign_bitwise_and, Right) | Op::infix(assign_bitwise_or, Right))
            .op(Op::infix(or, Left))
            .op(Op::infix(and, Left))
            .op(Op::infix(equal, Left) | Op::infix(not_equal, Left) | Op::infix(lower, Left)
                | Op::infix(lower_equal, Left) | Op::infix(greater, Left)
                | Op::infix(greater_equal, Left))
            .op(Op::infix(bitwise_or, Left))
            .op(Op::infix(xor, Left))
            .op(Op::infix(bitwise_and, Left))
            .op(Op::infix(lshift, Left) | Op::infix(rshift, Left))
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::infix(pow, Left))
            .op(Op::infix(map, Left) | Op::infix(filter, Left) | Op::infix(reduce, Left)
                | Op::postfix(sum) | Op::postfix(product) | Op::postfix(all)
                | Op::postfix(reduce_any) | Op::postfix(bitand_reduce) | Op::postfix(bitor_reduce))
            .op(Op::prefix(not) | Op::prefix(unary_minus) | Op::prefix(indirection))
            .op(Op::postfix(at) | Op::postfix(type_filter) | Op::postfix(function_call))
    };
}

#[macro_export]
macro_rules! unexpected {
    ($rule:expr) => {
        unreachable!("Unexpected rule: {:?}", $rule)
    };
}
