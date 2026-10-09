use std::sync::Arc;

use super::InstructionWithStr;
use super::local_variable::LocalVariables;
use crate::Error;
use crate::instruction::block::Block;
use crate::instruction::postfix_op::function_call;
use crate::instruction::set::Set;
use crate::instruction::{BaseInstruction, Exec, Instruction, Recreate};
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
            Rule::not => Not::create_instruction(rhs),
            Rule::unary_minus => UnaryMinus::create_instruction(rhs),
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

#[derive(Debug)]
pub struct UnaryMinus(Instruction);

impl UnaryMinus {
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
        Ok(UnaryMinus(instruction.instruction).into())
    }

    pub fn calc(variable: Variable) -> Variable {
        match variable {
            Variable::Int(num) => num.wrapping_neg().into(),
            Variable::Float(num) => Variable::Float(-num),
            operand => panic!("Tried to - {operand}"),
        }
    }
}

impl BaseInstruction for UnaryMinus {}

impl Exec for UnaryMinus {
    fn exec(&self, interpreter: &mut crate::Interpreter) -> super::ExecResult {
        let variable = self.0.exec(interpreter)?;
        Ok(Self::calc(variable))
    }
}

impl Recreate for UnaryMinus {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.0.recreate(local_variables);
        match instruction {
            Instruction::Variable(operand) => Self::calc(operand).into(),
            instruction => Self(instruction).into(),
        }
    }
}

impl ReturnType for UnaryMinus {
    fn return_type(&self) -> Type {
        self.0.return_type()
    }
}

#[derive(Debug)]
pub struct Not(Instruction);

lazy_static! {
    pub static ref ACCEPTED_NOT: Type = var_type!(int | bool);
}

impl Not {
    pub fn create_instruction(instruction: InstructionWithStr) -> Result<Instruction, Error> {
        let op = UnaryOperator::Not;
        let return_type = instruction.return_type();
        if !return_type.matches(&ACCEPTED_NOT) {
            return Err(Error::IncorectUnaryOperatorOperand {
                ins: instruction.str,
                op,
                expected: ACCEPTED_NOT.clone(),
                given: return_type,
            });
        }
        Ok(Not(instruction.instruction).into())
    }

    fn calc(variable: Variable) -> Variable {
        match variable {
            Variable::Bool(var) => (!var).into(),
            Variable::Int(num) => (!num).into(),
            operand => panic!("Tried to {} {operand}", stringify!(op2)),
        }
    }
}

impl BaseInstruction for Not {}

impl Exec for Not {
    fn exec(&self, interpreter: &mut crate::Interpreter) -> super::ExecResult {
        let variable = self.0.exec(interpreter)?;
        Ok(Self::calc(variable))
    }
}

impl Recreate for Not {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.0.recreate(local_variables);
        match instruction {
            Instruction::Variable(operand) => Self::calc(operand).into(),
            instruction => Self(instruction).into(),
        }
    }
}

impl ReturnType for Not {
    fn return_type(&self) -> Type {
        self.0.return_type()
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
