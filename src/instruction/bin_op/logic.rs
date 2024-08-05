use duplicate::duplicate_item;

#[duplicate_item(logic Logic symbol cond dv; [and] [And] [&&] [value!=0] [0]; [or] [Or] [||] [value==0] [1])]
pub mod logic {
    use crate::instruction::{
        can_be_used_int, BinOperation, BinOperator, Instruction, InstructionWithStr,
    };
    use crate::variable::{Array, ReturnType, Variable};
    use crate::Error;
    pub fn create_op(
        lhs: InstructionWithStr,
        rhs: InstructionWithStr,
    ) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !can_be_used_int(lhs_type.clone(), rhs_type.clone()) {
            return Err(Error::CannotDo2(lhs_type, stringify!(symbol), rhs_type));
        }
        Ok(create_from_instructions(lhs.instruction, rhs.instruction))
    }

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => exec(lhs, rhs).into(),
            (Instruction::Variable(Variable::Int(value)), instruction)
            | (instruction, Instruction::Variable(Variable::Int(value)))
                if cond =>
            {
                instruction
            }
            (lhs, rhs) => BinOperation {
                lhs,
                rhs,
                op: BinOperator::Logic,
            }
            .into(),
        }
    }

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (result, Variable::Int(value)) | (Variable::Int(value), result) if cond => result,
            (Variable::Array(array), _) | (_, Variable::Array(array)) => {
                Array::new_repeat(Variable::Int(dv), array.len()).into()
            }
            _ => Variable::Int(dv),
        }
    }
}
