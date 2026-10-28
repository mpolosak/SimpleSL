mod deref;
use super::InstructionWithStr;
use super::local_variable::LocalVariables;
use crate::instruction::prefix_op::deref::Deref;
use crate::instruction::{BaseInstruction, Exec, Instruction, Recreate};
use crate::unary_operator::UnaryOperator;
use crate::variable::{ReturnType, Type, Variable};
use crate::{self as simplesl, instruction::r#mut::Mut};
use crate::{Error, Interpreter};
use lazy_static::lazy_static;
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::{Rule, unexpected};

impl InstructionWithStr {
    pub fn create_prefix(op: Pair<'_, Rule>, rhs: Self, errors: &mut Vec<Error>) ->Self {
        let str = rhs.str.clone();
        let str = format!("{} {}", op.as_str(), str).into();
        let instruction = match op.as_rule() {
            Rule::not => Not::create_instruction(rhs, errors),
            Rule::unary_minus => UnaryMinus::create_instruction(rhs, errors),
            Rule::indirection => Deref::create_instruction(rhs, errors),
            Rule::r#mut => Mut::create_instruction(op, rhs, errors),
            rule => unexpected!(rule),
        };
        Self { instruction, str }
    }
}

lazy_static! {
    pub static ref ACCEPTED_NUM: Type = var_type!(int | float);
}

#[derive(Debug)]
pub struct UnaryMinus(Instruction);

impl UnaryMinus {
    pub fn create_instruction(instruction: InstructionWithStr, errors: &mut Vec<Error>) -> Instruction {
        let op = UnaryOperator::UnaryMinus;
        let return_type = instruction.return_type();
        if !return_type.matches(&ACCEPTED_NUM) {
            errors.push(Error::IncorectUnaryOperatorOperand {
                ins: instruction.str,
                op,
                expected: ACCEPTED_NUM.clone(),
                given: return_type,
            });
        }
        UnaryMinus(instruction.instruction).into()
    }
}

impl BaseInstruction for UnaryMinus {}

impl Exec for UnaryMinus {
    fn exec(&self, interpreter: &mut Interpreter) -> super::ExecResult {
        let variable = self.0.exec(interpreter)?;
        Ok(-variable)
    }
}

impl Recreate for UnaryMinus {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.0.recreate(local_variables);
        let Some(variable) = instruction.downcast_ref::<Variable>() else {
            return Self(instruction).into();
        };
        (-variable.clone()).into()
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
    pub fn create_instruction(instruction: InstructionWithStr, errors: &mut Vec<Error>) -> Instruction {
        let op = UnaryOperator::Not;
        let return_type = instruction.return_type();
        if !return_type.matches(&ACCEPTED_NOT) {
            errors.push(Error::IncorectUnaryOperatorOperand {
                ins: instruction.str,
                op,
                expected: ACCEPTED_NOT.clone(),
                given: return_type,
            });
        }
        Not(instruction.instruction).into()
    }
}

impl BaseInstruction for Not {}

impl Exec for Not {
    fn exec(&self, interpreter: &mut Interpreter) -> super::ExecResult {
        let variable = self.0.exec(interpreter)?;
        Ok(!variable)
    }
}

impl Recreate for Not {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.0.recreate(local_variables);
        let Some(variable) = instruction.downcast_ref::<Variable>() else {
            return Self(instruction).into();
        };
        (!variable.clone()).into()
    }
}

impl ReturnType for Not {
    fn return_type(&self) -> Type {
        self.0.return_type()
    }
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
            Err(Vec::from([Error::IncorectUnaryOperatorOperand {
                ins: "7.5".into(),
                op: UnaryOperator::Not,
                expected: var_type!(int | bool),
                given: var_type!(float)
            }]))
        );
        assert_eq!(parse_and_exec("!true"), Ok(var!(false)));
        assert_eq!(parse_and_exec("!false"), Ok(var!(true)));
        assert_eq!(
            parse_and_exec("![7, -4.5, 0]"),
            Err(Vec::from([Error::IncorectUnaryOperatorOperand {
                ins: "[7, -4.5, 0]".into(),
                op: UnaryOperator::Not,
                expected: var_type!(int | bool),
                given: var_type!([int | float])
            }]))
        );
        assert_eq!(
            parse_and_exec("-[7, true, 0]"),
            Err(Vec::from([Error::IncorectUnaryOperatorOperand {
                ins: "[7, true, 0]".into(),
                op: UnaryOperator::UnaryMinus,
                expected: var_type!(int | float),
                given: var_type!([int | bool])
            }]))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Vec<Error>> {
        Code::parse(&Interpreter::without_stdlib(), script).map(|code| code.exec().unwrap())
    }
}
