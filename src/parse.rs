use pest::pratt_parser::PrattParser;

#[derive(Parser)]
#[grammar = "simplesl.pest"]
pub struct SimpleSLParser;

lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::Left, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
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
            .op(Op::infix(map, Left) | Op::infix(filter, Left) | Op::infix(reduce, Left) | Op::postfix(sum) | Op::postfix(product))
            .op(Op::prefix(not) | Op::prefix(bitwise_not) | Op::prefix(unary_minus))
            .op(Op::postfix(at) | Op::postfix(type_filter) | Op::postfix(function_call))
    };
}

pub fn unexpected(rule: Rule) -> ! {
    unreachable!("Unexpected rule: {rule:?}")
}
