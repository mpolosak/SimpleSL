use super::{Instruction, InstructionWithStr, Loop};
use crate::{
    instruction::{
        control_flow::{if_else::return_type, IfElse},
        local_variable::LocalVariables,
    },
    variable::Type,
    Error,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
    instructions: &mut Vec<InstructionWithStr>,
) -> Result<(), Error> {
    let str = pair.as_str().into();
    let mut inner = pair.into_inner();
    let mut loop_instructions = Vec::<InstructionWithStr>::new();
    let pair = inner.next().unwrap();
    let condition_str = pair.as_str().into();
    InstructionWithStr::create(pair, local_variables, &mut loop_instructions)?;
    let return_type = return_type(instructions);
    if return_type != Type::Bool {
        return Err(Error::WrongCondition(condition_str, return_type));
    }
    let mut while_instructions = Vec::<InstructionWithStr>::new();
    let in_loop = local_variables.in_loop;
    local_variables.in_loop = true;
    let pair = inner.next().unwrap();
    InstructionWithStr::create(pair, local_variables, &mut while_instructions)?;
    local_variables.in_loop = in_loop;
    let if_else = InstructionWithStr {
        instruction: IfElse {
            if_true: while_instructions.into(),
            if_false: [InstructionWithStr {
                instruction: Instruction::Break,
                str: "Break".into(),
            }]
            .into(),
        }
        .into(),
        str: "if_else".into(),
    };
    loop_instructions.push(if_else);
    instructions.push(InstructionWithStr {
        instruction: Loop(loop_instructions.into()).into(),
        str,
    });
    Ok(())
}
