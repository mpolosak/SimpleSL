use super::{
    Instruction, InstructionWithStr, local_variable::LocalVariables,
};
use crate::{
    instruction::{block::Block, unary_operation::function_call},
    stdlib::operators::Slice,
};
use crate::{
    Error,
    instruction::set::Set,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::{Rule, unexpected};
use std::sync::Arc;

pub fn create(
    lhs: InstructionWithStr,
    pair: Pair<Rule>,
    local_variables: &LocalVariables,
) -> Result<Instruction, Error> {
    let lhs_type = lhs.return_type();
    if !lhs_type.can_be_indexed() {
        return Err(Error::CannotSlice(lhs.str, lhs_type));
    }
    let inner = pair.into_inner();
    if inner.peek().is_none() {
        return Ok(lhs.instruction);
    }
    let mut start = None;
    let mut stop = None;
    let mut step = None;
    for pair in inner {
        let rule = pair.as_rule();
        let instruction = InstructionWithStr::new_expression(pair, local_variables)?;
        if instruction.return_type() != Type::Int {
            return Err(Error::CannotIndexWith(instruction.str));
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
    
    let variable_set = Set::new_ident("variable".into(), lhs.instruction).into();
    let start_set = Set::new_ident("start".into(), start).into();
    let stop_set = Set::new_ident("end".into(), stop).into();
    let step_set = Set::new_ident("step".into(), step).into();

    let mut function = Arc::unwrap_or_clone(Variable::from(Slice).into_function().unwrap());
    function.return_type = variable_type;
    let call = function_call(function);

    Ok(Block {
        instructions: [variable_set, start_set, stop_set, step_set, call].into(),
    }
    .into())
}

#[cfg(test)]
mod test {
    use crate::{self as simplesl, Code, Error, Interpreter, variable::Variable};
    use simplesl_macros::var;

    #[test]
    fn slicing() {
        assert_eq!(parse_and_exec("[15, 45, 16][::]"), Ok(var!([15, 45, 16])));
        assert_eq!(parse_and_exec("[15, 45, 16][::1]"), Ok(var!([15, 45, 16])));
        assert_eq!(parse_and_exec("[15, 45, 16][:]"), Ok(var!([15, 45, 16])));
        assert_eq!(parse_and_exec("[15, 45, 16][1:]"), Ok(var!([45, 16])));
        assert_eq!(parse_and_exec("[15, 45, 16][1::]"), Ok(var!([45, 16])));
        assert_eq!(parse_and_exec("[15, 45, 16][1::1]"), Ok(var!([45, 16])));
        assert_eq!(parse_and_exec("[15, 45, 16][1:-1:]"), Ok(var!([45])));
        assert_eq!(parse_and_exec("[15, 45, 16][1:-1:1]"), Ok(var!([45])));
        assert_eq!(parse_and_exec("[15, 45, 16][1:-1]"), Ok(var!([45])));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][::2]"), Ok(var!([15, 16])));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1::2]"), Ok(var!([45, 0])));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1:-1:2]"), Ok(var!([45])));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1:2:2]"), Ok(var!([45])));
        assert_eq!(parse_and_exec("[15, 45, 16][::-1]"), Ok(var!([16, 45, 15])));
        assert_eq!(parse_and_exec("[15, 45, 16][1::-1]"), Ok(var!([45, 15])));
        assert_eq!(parse_and_exec("[15, 45, 16][1:-1:-1]"), Ok(var!([])));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][::-2]"), Ok(var!([0, 45])));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1::-2]"), Ok(var!([45])));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1::-6]"), Ok(var!([45])));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1:-1:-2]"), Ok(var!([])));
        assert_eq!(parse_and_exec("[15, 45, 16, 0][1:2:-2]"), Ok(var!([])));
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
