use std::sync::Arc;

use super::InstructionWithStr;
use crate::Error;
use crate::instruction::Instruction;
use crate::instruction::block::Block;
use crate::instruction::set::Set;
use crate::instruction::unary_operation::function_call;
use crate::stdlib::operators::Deref;
use crate::unary_operator::UnaryOperator;
use crate::variable::{ReturnType, Type, Variable};
use crate::{self as simplesl, instruction::r#mut::Mut};
use lazy_static::lazy_static;
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::{Rule, unexpected};

impl InstructionWithStr {
    pub fn create_prefix(op: Pair<'_, Rule>, rhs: Self) -> Result<Self, Error> {
        let str = rhs.str.clone();
        let str = format!("{} {}", op.as_str(), str).into();
        let instruction = match op.as_rule() {
            Rule::not => not::create_instruction(rhs),
            Rule::unary_minus => unary_minus::create_instruction(rhs),
            Rule::indirection => create_deref(rhs),
            Rule::r#mut => Mut::create_instruction(op, rhs),
            rule => unexpected!(rule),
        }?;
        Ok(Self { instruction, str })
    }
}

lazy_static! {
    pub static ref ACCEPTED_NUM: Type = var_type!(int | float);
}

pub mod unary_minus {
    use crate::{
        self as simplesl, Error,
        instruction::{Instruction, InstructionWithStr, unary_operation::UnaryOperation},
        unary_operator::UnaryOperator,
        variable::{ReturnType, Variable},
    };
    use match_any::match_any;
    use simplesl_macros::var;

    use super::ACCEPTED_NUM;

    pub fn create_instruction(instruction: InstructionWithStr) -> Result<Instruction, Error> {
        let op = UnaryOperator::UnaryMinus;
        let return_type = instruction.return_type();
        if !return_type.matches(&ACCEPTED_NUM) {
            return Err(Error::IncorectUnaryOperatorOperand {
                ins: instruction.str,
                op,
                expected: ACCEPTED_NUM.clone(),
                given: return_type,
            });
        }
        Ok(UnaryOperation {
            instruction: instruction.instruction,
            op,
        }
        .into())
    }

    pub fn create_from_instruction(instruction: Instruction) -> Instruction {
        match_any! { instruction,
            Instruction::Variable(operand) => exec(operand).into(),
            instruction => UnaryOperation {instruction,op:UnaryOperator::UnaryMinus }.into()
        }
    }

    pub fn exec(variable: Variable) -> Variable {
        match variable {
            Variable::Int(num) => num.wrapping_neg().into(),
            Variable::Float(num) => var!(-num),
            operand => panic!("Tried to - {operand}"),
        }
    }
}

pub mod not {
    use crate::{
        self as simplesl, Error,
        instruction::{Instruction, InstructionWithStr, unary_operation::UnaryOperation},
        unary_operator::UnaryOperator,
        variable::{ReturnType, Type, Variable},
    };
    use lazy_static::lazy_static;
    use match_any::match_any;
    use simplesl_macros::var_type;

    lazy_static! {
        pub static ref ACCEPTED: Type = var_type!(int | bool);
    }

    pub fn create_instruction(instruction: InstructionWithStr) -> Result<Instruction, Error> {
        let op = UnaryOperator::Not;
        let return_type = instruction.return_type();
        if !return_type.matches(&ACCEPTED) {
            return Err(Error::IncorectUnaryOperatorOperand {
                ins: instruction.str,
                op,
                expected: ACCEPTED.clone(),
                given: return_type,
            });
        }
        Ok(UnaryOperation {
            instruction: instruction.instruction,
            op,
        }
        .into())
    }

    pub fn create_from_instruction(instruction: Instruction) -> Instruction {
        match_any! { instruction,
            Instruction::Variable(operand) => exec(operand).into(),
            instruction => UnaryOperation {instruction, op: UnaryOperator::Not } .into()
        }
    }
    pub fn exec(variable: Variable) -> Variable {
        match variable {
            Variable::Bool(var) => (!var).into(),
            Variable::Int(num) => (!num).into(),
            operand => panic!("Tried to {} {operand}", stringify!(op2)),
        }
    }
}

pub fn create_deref(instruction: InstructionWithStr) -> Result<Instruction, Error> {
    let op = UnaryOperator::Indirection;
    let return_type = instruction.return_type();
    let Some(return_type) = return_type.mut_element_type() else {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: instruction.str,
            op,
            expected: Type::Mut(Type::Any.into()),
            given: return_type,
        });
    };

    let variable_set = Set::new_ident("variable".into(), instruction.instruction).into();

    let mut function = Arc::unwrap_or_clone(Variable::from(Deref).into_function().unwrap());
    function.return_type = return_type;
    let call = function_call(function);

    Ok(Block {
        instructions: [variable_set, call].into(),
    }
    .into())
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, Interpreter, unary_operator::UnaryOperator,
        variable::Variable,
    };
    use simplesl_macros::{var, var_type};

    #[test]
    fn prefix_ops() {
        assert_eq!(parse_and_exec("-5"), Ok(var!(-5)));
        assert_eq!(parse_and_exec("-7.5"), Ok(var!(-7.5)));
        assert_eq!(parse_and_exec("!5"), Ok(var!(-6)));
        assert_eq!(parse_and_exec("!0"), Ok(var!(-1)));
        assert_eq!(
            parse_and_exec("!7.5"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "7.5".into(),
                op: UnaryOperator::Not,
                expected: var_type!(int | bool),
                given: var_type!(float)
            })
        );
        assert_eq!(parse_and_exec("!true"), Ok(var!(false)));
        assert_eq!(parse_and_exec("!false"), Ok(var!(true)));
        assert_eq!(
            parse_and_exec("![7, -4.5, 0]"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[7, -4.5, 0]".into(),
                op: UnaryOperator::Not,
                expected: var_type!(int | bool),
                given: var_type!([int | float])
            })
        );
        assert_eq!(
            parse_and_exec("-[7, true, 0]"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[7, true, 0]".into(),
                op: UnaryOperator::UnaryMinus,
                expected: var_type!(int | float),
                given: var_type!([int | bool])
            })
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, crate::Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
