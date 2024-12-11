use super::{
    at,
    function::call,
    local_variable::LocalVariables,
    prefix_op::{indirection, not, unary_minus},
    reduce::{self, bool_reduce, collect, product, sum},
    type_filter::TypeFilter,
    Exec, ExecResult, ExecStop, Instruction, InstructionWithStr, Recreate,
};
use crate::{
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type},
    Error, Interpreter,
};
use pest::iterators::Pair;
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
            Rule::all => bool_reduce::create(lhs, UnaryOperator::All),
            Rule::reduce_any => bool_reduce::create(lhs, UnaryOperator::Any),
            Rule::bitand_reduce => reduce::bit::create(lhs, UnaryOperator::BitAnd),
            Rule::bitor_reduce => reduce::bit::create(lhs, UnaryOperator::BitOr),
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
            UnaryOperator::All => bool_reduce::all(var, interpreter)?,
            UnaryOperator::Any => bool_reduce::any(var, interpreter)?,
            UnaryOperator::BitAnd => reduce::bit::and(var, interpreter)?,
            UnaryOperator::BitOr => reduce::bit::or(var, interpreter)?,
            UnaryOperator::Sum => sum::exec(var, interpreter)?,
            UnaryOperator::Product => product::exec(var, interpreter)?,
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
            UnaryOperator::Not => not::create_from_instruction(instruction),
            UnaryOperator::UnaryMinus => unary_minus::create_from_instruction(instruction),
            op => UnaryOperation { instruction, op }.into(),
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
