use super::{Instruction, InstructionWithSpan};
use crate::errors::{CannotIndexWith, CannotSlice};
use crate::instruction::parsing_context::ParsingContext;
use crate::stdlib::operators::Slice;
use crate::{
    Error,
    instruction::{function::call::FunctionCall, tuple::Tuple},
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::{Rule, unexpected};
use std::sync::Arc;

pub fn create(
    lhs: InstructionWithSpan,
    pair: Pair<Rule>,
    parsing_context: &ParsingContext,
    errors: &mut Vec<Error>,
) -> Instruction {
    let lhs_type = lhs.return_type();
    if !lhs_type.can_be_indexed() {
        let ins = parsing_context.slice(lhs.span).into();
        errors.push(
            CannotSlice {
                ins,
                var_type: lhs_type,
            }
            .into(),
        );
    }
    let inner = pair.into_inner();
    if inner.peek().is_none() {
        return lhs.instruction;
    }
    let mut start = None;
    let mut stop = None;
    let mut step = None;
    for pair in inner {
        let rule = pair.as_rule();
        let instruction = InstructionWithSpan::new_expression(pair, parsing_context, errors);
        if instruction.return_type() != Type::Int {
            let ins = parsing_context.slice(instruction.span).into();
            errors.push(CannotIndexWith { ins }.into());
        }
        match rule {
            Rule::start => start = Some(instruction.instruction),
            Rule::step => step = Some(instruction.instruction),
            Rule::stop => stop = Some(instruction.instruction),
            rule => unexpected!(rule),
        }
    }

    let start = start.unwrap_or_else(|| Variable::none().into());
    let stop = stop.unwrap_or_else(|| Variable::none().into());
    let step = step.unwrap_or_else(|| Variable::none().into());

    let variable_type = lhs.return_type();

    let mut function = Arc::unwrap_or_clone(Variable::from(Slice).into_function().unwrap());
    function.return_type = variable_type;

    let function = Variable::from(function).into();
    let args = Tuple {
        elements: [lhs.instruction, start, stop, step].into(),
    }
    .into();

    FunctionCall { function, args }.into()
}

#[cfg(test)]
mod test {
    use crate::{self as simplesl, Code, Interpreter, variable::Variable};
    use simplesl_macros::var;

    #[test]
    fn slicing() {
        assert_eq!(parse_and_exec("[15, 45, 16][::]"), var!([15, 45, 16]));
        assert_eq!(parse_and_exec("[15, 45, 16][::1]"), var!([15, 45, 16]));
        assert_eq!(parse_and_exec("[15, 45, 16][:]"), var!([15, 45, 16]));
        assert_eq!(parse_and_exec("[15, 45, 16][1:]"), var!([45, 16]));
        assert_eq!(parse_and_exec("[15, 45, 16][1::]"), var!([45, 16]));
        assert_eq!(parse_and_exec("[15, 45, 16][1::1]"), var!([45, 16]));
        assert_eq!(parse_and_exec("[15, 45, 16][1:-1:]"), var!([45]));
        assert_eq!(parse_and_exec("[15, 45, 16][1:-1:1]"), var!([45]));
        assert_eq!(parse_and_exec("[15, 45, 16][1:-1]"), var!([45]));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][::2]"), var!([15, 16]));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1::2]"), var!([45, 0]));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1:-1:2]"), var!([45]));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1:2:2]"), var!([45]));
        assert_eq!(parse_and_exec("[15, 45, 16][::-1]"), var!([16, 45, 15]));
        assert_eq!(parse_and_exec("[15, 45, 16][1::-1]"), var!([45, 15]));
        assert_eq!(parse_and_exec("[15, 45, 16][1:-1:-1]"), var!([]));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][::-2]"), var!([0, 45]));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1::-2]"), var!([45]));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1::-6]"), var!([45]));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1:-1:-2]"), var!([]));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1:2:-2]"), var!([]));
    }

    fn parse_and_exec(script: &str) -> Variable {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
            .unwrap()
    }
}
