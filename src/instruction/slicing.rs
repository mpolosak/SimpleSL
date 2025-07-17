use std::sync::Arc;

use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
};
use crate::{
    Error, ExecError, Interpreter,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Slicing {
    lhs: InstructionWithStr,
    start: Option<InstructionWithStr>,
    stop: Option<InstructionWithStr>,
    step: Option<InstructionWithStr>,
}

impl Slicing {
    pub fn create(
        lhs: InstructionWithStr,
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        if !lhs_type.can_be_indexed() {
            return Err(Error::CannotSlice(lhs.str, lhs_type));
        }
        let mut inner = pair.into_inner();
        let (start, stop, step) = match (inner.next(), inner.next(), inner.next()) {
            (Some(start), Some(stop), Some(step)) => (
                Some(InstructionWithStr::new_expression(start, local_variables)?),
                Some(InstructionWithStr::new_expression(stop, local_variables)?),
                Some(InstructionWithStr::new_expression(step, local_variables)?),
            ),
            (Some(start), Some(stop), None)
                if start.as_rule() == Rule::start && stop.as_rule() == Rule::stop =>
            {
                (
                    Some(InstructionWithStr::new_expression(start, local_variables)?),
                    Some(InstructionWithStr::new_expression(stop, local_variables)?),
                    None,
                )
            }
            (Some(start), Some(step), None) if start.as_rule() == Rule::start => (
                Some(InstructionWithStr::new_expression(start, local_variables)?),
                None,
                Some(InstructionWithStr::new_expression(step, local_variables)?),
            ),
            (Some(stop), Some(step), None) => (
                None,
                Some(InstructionWithStr::new_expression(stop, local_variables)?),
                Some(InstructionWithStr::new_expression(step, local_variables)?),
            ),
            (Some(start), None, None) if start.as_rule() == Rule::start => (
                Some(InstructionWithStr::new_expression(start, local_variables)?),
                None,
                None,
            ),
            (Some(stop), None, None) if stop.as_rule() == Rule::stop => (
                None,
                Some(InstructionWithStr::new_expression(stop, local_variables)?),
                None,
            ),
            (Some(step), None, None) => (
                None,
                None,
                Some(InstructionWithStr::new_expression(step, local_variables)?),
            ),
            _ => return Ok(lhs.instruction),
        };
        if let (Some(index), _, _) | (_, Some(index), _) | (_, _, Some(index)) =
            (&start, &stop, &step)
            && index.return_type() != Type::Int
        {
            return Err(Error::CannotIndexWith(index.str.clone()));
        }

        Ok(Self {
            lhs,
            start,
            stop,
            step,
        }
        .into())
    }

    fn exec_index(
        index: &Option<InstructionWithStr>,
        interpreter: &mut Interpreter,
    ) -> Result<Option<isize>, super::ExecStop> {
        let exec = |ins: &InstructionWithStr| ins.exec(interpreter);
        let start = index
            .as_ref()
            .map(exec)
            .transpose()?
            .map(Variable::into_int)
            .transpose()
            .unwrap()
            .map(|i| i as isize);
        Ok(start)
    }
}

impl Exec for Slicing {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;

        let start = Slicing::exec_index(&self.start, interpreter)?.into();
        let end = Slicing::exec_index(&self.stop, interpreter)?.into();
        let step = Slicing::exec_index(&self.step, interpreter)?;

        let s = slyce::Slice { start, end, step };

        if let Variable::String(lhs) = lhs {
            let chars: Box<[char]> = lhs.chars().collect();
            let result: String = s.apply(&chars).cloned().collect();
            return Ok(result.into());
        }

        let array = lhs.into_array().unwrap();
        let result: Arc<[Variable]> = s.apply(array.as_ref()).cloned().collect();
        Ok(result.into())
    }
}

impl Recreate for Slicing {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let lhs = self.lhs.recreate(local_variables)?;
        let start = self
            .start
            .as_ref()
            .map(|iws| iws.recreate(local_variables))
            .transpose()?;
        let stop = self
            .stop
            .as_ref()
            .map(|iws| iws.recreate(local_variables))
            .transpose()?;
        let step = self
            .step
            .as_ref()
            .map(|iws| iws.recreate(local_variables))
            .transpose()?;
        Ok(Self {
            lhs,
            start,
            stop,
            step,
        }
        .into())
    }
}

impl ReturnType for Slicing {
    fn return_type(&self) -> Type {
        self.lhs
            .return_type()
            .element_type()
            .unwrap_or(Type::String)
    }
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
