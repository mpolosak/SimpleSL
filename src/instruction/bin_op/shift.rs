use duplicate::duplicate_item;

#[duplicate_item(
    shift Shift op1 op2;
    [lshift] [LShift] [lhs << rhs] [>>]; [rshift] [RShift] [lhs >> rhs] [>>];
)]
pub mod shift {
    use std::sync::Arc;

    use crate::{
        instruction::{BinOperation, Instruction},
        variable::{Array, Variable},
        BinOperator, ExecError,
    };

    pub fn create_from_instructions(
        lhs: Instruction,
        rhs: Instruction,
    ) -> Result<Instruction, ExecError> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Ok(exec(lhs, rhs)?.into()),
            (_, Instruction::Variable(Variable::Int(rhs))) if !(0..=63).contains(&rhs) => {
                Err(ExecError::OverflowShift)
            }
            (Instruction::ArrayRepeat(array), rhs) => Arc::unwrap_or_clone(array)
                .try_map(|lhs| create_from_instructions(lhs, rhs.clone()))
                .map(Instruction::from),
            (lhs, Instruction::ArrayRepeat(array)) => Arc::unwrap_or_clone(array)
                .try_map(|rhs| create_from_instructions(lhs.clone(), rhs))
                .map(Instruction::from),
            (lhs, rhs) => Ok(BinOperation {
                lhs,
                rhs,
                op: BinOperator::Shift,
            }
            .into()),
        }
    }

    pub fn exec(lhs: Variable, rhs: Variable) -> Result<Variable, ExecError> {
        match (lhs, rhs) {
            (_, Variable::Int(rhs)) if !(0..=63).contains(&rhs) => Err(ExecError::OverflowShift),
            (Variable::Int(lhs), Variable::Int(rhs)) => Ok((op1).into()),
            (value, Variable::Array(array)) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|rhs| exec(value.clone(), rhs))
                    .collect::<Result<Arc<_>, _>>()?;
                let element_type = array.element_type().clone();
                Ok(Array {
                    element_type,
                    elements,
                }
                .into())
            }
            (Variable::Array(array), value) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|lhs| exec(lhs, value.clone()))
                    .collect::<Result<Arc<_>, _>>()?;
                let element_type = array.element_type().clone();
                Ok(Array {
                    element_type,
                    elements,
                }
                .into())
            }
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                stringify!(op2)
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{variable::Variable, Code, Error, Interpreter};
    use proptest::proptest;

    #[test]
    pub fn lshift() {
        assert_eq!(parse_and_exec("4 << 5"), Ok(Variable::Int(4 << 5)));
        assert_eq!(parse_and_exec("0 << 4"), Ok(Variable::Int(0 << 4)));
        assert_eq!(
            parse_and_exec("[45, 15, 56, 67] << 4"),
            Ok([
                Variable::Int(45 << 4),
                Variable::Int(15 << 4),
                Variable::Int(56 << 4),
                Variable::Int(67 << 4)
            ]
            .into())
        );
        assert_eq!(
            parse_and_exec("15 << [1, 2, 3, 4]"),
            Ok([
                Variable::Int(15 << 1),
                Variable::Int(15 << 2),
                Variable::Int(15 << 3),
                Variable::Int(15 << 4)
            ]
            .into())
        );
        assert_eq!(parse_and_exec("45 << 64"), Err(Error::OverflowShift));
        assert_eq!(parse_and_exec("45 >> 90"), Err(Error::OverflowShift))
    }

    #[test]
    pub fn rshift() {
        assert_eq!(parse_and_exec("4 >> 5"), Ok(Variable::Int(4 >> 5)));
        assert_eq!(parse_and_exec("0 >> 4"), Ok(Variable::Int(0 >> 4)));
        assert_eq!(
            parse_and_exec("[45, 15, 56, 67] >> 4"),
            Ok([
                Variable::Int(45 >> 4),
                Variable::Int(15 >> 4),
                Variable::Int(56 >> 4),
                Variable::Int(67 >> 4)
            ]
            .into())
        );
        assert_eq!(
            parse_and_exec("15 >> [1, 2, 3, 4]"),
            Ok([
                Variable::Int(15 >> 1),
                Variable::Int(15 >> 2),
                Variable::Int(15 >> 3),
                Variable::Int(15 >> 4)
            ]
            .into())
        );
        assert_eq!(parse_and_exec("45 >> 64"), Err(Error::OverflowShift));
        assert_eq!(parse_and_exec("45 >> 90"), Err(Error::OverflowShift))
    }

    proptest! {
        #[test]
        fn shift_doesnt_crash(a: i64, b: i64){
            let _ = parse_and_exec(&format!("{a} << {b}"));
            let _ = parse_and_exec(&format!("{a} >> {b}"));
        }
    }

    fn parse_and_exec(script: &str) -> Result<Variable, crate::Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
