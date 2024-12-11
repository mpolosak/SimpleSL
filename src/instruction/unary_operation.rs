use super::{
    at,
    function::call,
    local_variable::LocalVariables,
    prefix_op::{indirection, not, unary_minus},
    reduce::{all, any, bitand, bitor, collect, product, sum},
    type_filter::TypeFilter,
    Exec, ExecResult, ExecStop, Instruction, InstructionWithStr, Recreate,
};
use crate as simplesl;
use crate::{
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type},
    Error, Interpreter,
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::{unexpected, Rule};

impl InstructionWithStr {
    pub fn create_postfix(
        op: Pair<'_, Rule>,
        lhs: Self,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Self, Error> {
        let str = format!("{} {}", lhs.str, op.as_str()).into();
        let instruction = match op.as_rule() {
            Rule::at => at::create(lhs, op, local_variables),
            Rule::type_filter => {
                TypeFilter::create_instruction(lhs, op.into_inner().next().unwrap())
            }
            Rule::function_call => call::create_instruction(lhs, op, local_variables),
            Rule::sum => sum::create(lhs),
            Rule::product => product::create(lhs),
            Rule::all => create_bool_reduce(lhs, UnaryOperator::All),
            Rule::reduce_any => create_bool_reduce(lhs, UnaryOperator::Any),
            Rule::bitand_reduce => create_bit_reduce(lhs, UnaryOperator::BitAnd),
            Rule::bitor_reduce => create_bit_reduce(lhs, UnaryOperator::BitOr),
            Rule::collect => collect::create(lhs),
            rule => unexpected!(rule),
        }?;
        Ok(Self { instruction, str })
    }
}

#[derive(Debug)]
pub struct UnaryOperation {
    pub instruction: Instruction,
    pub op: UnaryOperator,
}

impl Exec for UnaryOperation {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let var = self.instruction.exec(interpreter)?;
        Ok(match self.op {
            UnaryOperator::All => all::exec(var),
            UnaryOperator::Any => any::exec(var),
            UnaryOperator::BitAnd => bitand::exec(var),
            UnaryOperator::BitOr => bitor::exec(var),
            UnaryOperator::Sum => sum::exec(var),
            UnaryOperator::Product => product::exec(var),
            UnaryOperator::Not => not::exec(var),
            UnaryOperator::UnaryMinus => unary_minus::exec(var),
            UnaryOperator::Return => return Err(ExecStop::Return(var)),
            UnaryOperator::Indirection => indirection::exec(var),
            UnaryOperator::FunctionCall => var.into_function().unwrap().exec(interpreter)?,
            UnaryOperator::Collect => collect::exec(var, interpreter)?,
        })
    }
}

impl Recreate for UnaryOperation {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
    ) -> Result<super::Instruction, crate::ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        Ok(match self.op {
            UnaryOperator::All => all::recreate(instruction),
            UnaryOperator::Any => any::recreate(instruction),
            UnaryOperator::BitAnd => bitand::recreate(instruction),
            UnaryOperator::BitOr => bitor::recreate(instruction),
            UnaryOperator::Sum => sum::recreate(instruction),
            UnaryOperator::Product => product::recreate(instruction),
            UnaryOperator::Not => not::create_from_instruction(instruction),
            UnaryOperator::UnaryMinus => unary_minus::create_from_instruction(instruction),
            op @ (UnaryOperator::Return
            | UnaryOperator::Indirection
            | UnaryOperator::FunctionCall
            | UnaryOperator::Collect) => UnaryOperation { instruction, op }.into(),
        })
    }
}

impl ReturnType for UnaryOperation {
    fn return_type(&self) -> Type {
        let return_type = self.instruction.return_type();
        match self.op {
            UnaryOperator::All | UnaryOperator::Any => Type::Bool,
            UnaryOperator::BitAnd | UnaryOperator::BitOr => Type::Int,
            UnaryOperator::Sum | UnaryOperator::Product => return_type.element_type().unwrap(),
            UnaryOperator::Not | UnaryOperator::UnaryMinus => return_type,
            UnaryOperator::Return => Type::Never,
            UnaryOperator::Indirection => indirection::return_type(return_type),
            UnaryOperator::FunctionCall => return_type.return_type().unwrap(),
            UnaryOperator::Collect => collect::return_type(return_type),
        }
    }
}

pub fn create_bit_reduce(
    array: InstructionWithStr,
    op: UnaryOperator,
) -> Result<Instruction, Error> {
    let return_type = array.return_type();
    if !return_type.matches(&var_type!([int] | [bool])) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: array.str,
            op,
            expected: var_type!([int] | [bool]),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: array.instruction,
        op,
    }
    .into())
}

pub fn create_bool_reduce(
    array: InstructionWithStr,
    op: UnaryOperator,
) -> Result<Instruction, Error> {
    let return_type = array.return_type();
    if !return_type.matches(&var_type!([bool])) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: array.str,
            op,
            expected: var_type!([bool]),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: array.instruction,
        op,
    }
    .into())
}
