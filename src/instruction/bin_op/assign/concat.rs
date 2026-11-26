use crate::{
    BinOperator,
    errors::{CannotDo2, ErrorWithRange},
    instruction::{
        BaseInstruction, Exec, Instruction, InstructionWithRange, bin_op::concat::Concat,
    },
    range_from_2_ranges,
    variable::{ReturnType, Type},
};

#[derive(Debug)]
pub struct AssignConcat {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

impl BaseInstruction for AssignConcat {}

impl AssignConcat {
    pub fn create_instruction(
        lhs: InstructionWithRange,
        rhs: InstructionWithRange,
        errors: &mut Vec<ErrorWithRange>,
    ) -> Instruction {
        let lhs_rt = lhs.return_type();
        let rhs_rt = rhs.return_type();
        if !Self::can_be_used(&lhs_rt, &rhs_rt) {
            let error = CannotDo2 {
                lhs: lhs_rt,
                op: BinOperator::AssignAdd,
                rhs: rhs_rt,
            }
            .into();
            let span = range_from_2_ranges(&lhs.range, &rhs.range);
            errors.push(ErrorWithRange { error, range: span });
        }
        Self {
            lhs: lhs.instruction,
            rhs: rhs.instruction,
        }
        .into()
    }

    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let Some(element_type) = lhs.mut_element_type() else {
            return false;
        };
        rhs.matches(&element_type)
    }
}

impl Exec for AssignConcat {
    fn exec(&self, interpreter: &mut crate::Interpreter) -> crate::instruction::ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        let lhs = lhs.into_mut().unwrap();
        let mut lhs = lhs.variable.write().unwrap();
        *lhs = Concat::calc(lhs.clone(), rhs);
        Ok(lhs.clone())
    }
}

impl ReturnType for AssignConcat {
    fn return_type(&self) -> Type {
        self.lhs
            .return_type()
            .mut_element_type()
            .unwrap_or(Type::Never)
    }
}
