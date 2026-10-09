mod iter;
use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, at,
    function::call,
    local_variable::LocalVariables,
    reduce::{self, bool_reduce, collect, product, sum},
    tuple_access,
    type_filter::TypeFilter,
};
use crate::{
    Error, Interpreter,
    instruction::{BaseInstruction, ExecStop, field_access, postfix_op::iter::Iter, slicing},
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::{Rule, unexpected};

impl InstructionWithStr {
    pub fn create_postfix(
        op: Pair<'_, Rule>,
        lhs: Self,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Self, Error> {
        let str = format!("{} {}", lhs.str, op.as_str()).into();
        let instruction = match op.as_rule() {
            Rule::at => at::create(lhs.instruction, op, local_variables),
            Rule::type_filter => {
                TypeFilter::create_instruction(lhs.instruction, op.into_inner().next().unwrap())
            }
            Rule::function_call => call::create_instruction(lhs, op, local_variables),
            Rule::tuple_access => tuple_access::create_instruction(lhs, op),
            Rule::field_access => field_access::create_instruction(lhs, op),
            Rule::sum => sum::create(lhs),
            Rule::product => product::create(lhs),
            Rule::all => bool_reduce::create(lhs, UnaryOperator::All),
            Rule::reduce_any => bool_reduce::create(lhs, UnaryOperator::Any),
            Rule::bitand_reduce => reduce::bit::create(lhs, UnaryOperator::BitAnd),
            Rule::bitor_reduce => reduce::bit::create(lhs, UnaryOperator::BitOr),
            Rule::collect => collect::create(lhs),
            Rule::iter => Iter::create_instruction(lhs),
            Rule::slicing => slicing::create(lhs, op, local_variables),
            rule => unexpected!(rule),
        }?;
        Ok(Self { instruction, str })
    }
}

#[derive(Debug)]
pub struct UnaryFunctionCall(pub Instruction);

impl BaseInstruction for UnaryFunctionCall {}

impl Exec for UnaryFunctionCall {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let var = self.0.exec(interpreter)?;
        var.into_function()
            .unwrap()
            .exec(interpreter)
            .map_err(ExecStop::from)
    }
}

impl Recreate for UnaryFunctionCall {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.0.recreate(local_variables);
        Self(instruction).into()
    }
}

impl ReturnType for UnaryFunctionCall {
    fn return_type(&self) -> Type {
        self.0.return_type().return_type().unwrap()
    }
}

pub fn function_call<T: Into<Variable>>(function: T) -> Instruction {
    UnaryFunctionCall(function.into().into()).into()
}
