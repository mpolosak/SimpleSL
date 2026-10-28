use super::{Instruction, InstructionWithStr, local_variable::LocalVariables};
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
    lhs: InstructionWithStr,
    pair: Pair<Rule>,
    local_variables: &LocalVariables,
    errors: &mut Vec<Error>
) -> Instruction {
    let lhs_type = lhs.return_type();
    if !lhs_type.can_be_indexed() {
        errors.push(Error::CannotSlice(lhs.str.clone(), lhs_type));
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
        let instruction = InstructionWithStr::new_expression(pair, local_variables, errors);
        if instruction.return_type() != Type::Int {
            errors.push(Error::CannotIndexWith(instruction.str));
        }
        match rule {
            Rule::start => start = Some(instruction.instruction),
            Rule::step => step = Some(instruction.instruction),
            Rule::stop => stop = Some(instruction.instruction),
            rule => unexpected!(rule),
        }
    }

    let start = start.unwrap_or_else(|| Variable::Void.into());
    let stop = stop.unwrap_or_else(|| Variable::Void.into());
    let step = step.unwrap_or_else(|| Variable::Void.into());

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
        Code::parse(&Interpreter::without_stdlib(), script).unwrap().exec().unwrap()
    }
}
