use crate::{
    Error,
    instruction::{
        Instruction, InstructionWithStr, Loop, control_flow::IfElse, local_variable::LocalVariables,
    },
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
) -> Result<Instruction, Error> {
    let mut inner = pair.into_inner();
    let condition = InstructionWithStr::new_expression(inner.next().unwrap(), local_variables)?;
    let return_type = condition.return_type();
    if return_type != Type::Bool {
        return Err(Error::WrongCondition(condition.str, return_type));
    }
    let in_loop = local_variables.in_loop;
    local_variables.in_loop = true;
    let instruction = InstructionWithStr::new(inner.next().unwrap(), local_variables)?;
    local_variables.in_loop = in_loop;
    if let Instruction::Variable(value) = condition.instruction {
        return if value == Variable::Bool(true) {
            Ok(Loop(instruction).into())
        } else {
            Ok(Variable::Void.into())
        };
    }
    let str = format!("if {} {} else break", condition.str, instruction.str).into();
    let instruction = InstructionWithStr {
        instruction: IfElse {
            condition,
            if_true: instruction,
            if_false: InstructionWithStr {
                instruction: Instruction::Break,
                str: "Break".into(),
            },
        }
        .into(),
        str,
    };
    Ok(Loop(instruction).into())
}
