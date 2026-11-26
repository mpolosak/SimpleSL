use crate::{
    BinOperator, Interpreter,
    errors::{CannotDo2, ErrorWithSpan},
    instruction::{
        BaseInstruction, Exec, ExecResult, InstructionWithSpan,
        bin_op::{add::Add, assign::concat::AssignConcat},
    },
    source_span_from_2_spans,
};
use crate::{
    instruction::Instruction,
    variable::{ReturnType, Type},
};

#[derive(Debug)]
pub struct AssignAdd {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

impl BaseInstruction for AssignAdd {}

impl AssignAdd {
    pub fn create_instruction(
        lhs: InstructionWithSpan,
        rhs: InstructionWithSpan,
        errors: &mut Vec<ErrorWithSpan>,
    ) -> Instruction {
        let lhs_rt = lhs.return_type();
        let rhs_rt = rhs.return_type();
        if rhs_rt.is_array() {
            return AssignConcat::create_instruction(lhs, rhs, errors);
        }
        if !Self::can_be_used(&lhs_rt, &rhs_rt) {
            let error = CannotDo2 {
                lhs: lhs_rt,
                op: BinOperator::AssignAdd,
                rhs: rhs_rt,
            }
            .into();
            let span = source_span_from_2_spans(&lhs.span, &rhs.span);
            errors.push(ErrorWithSpan { error, span });
        }
        Self {
            lhs: lhs.instruction,
            rhs: rhs.instruction,
        }
        .into()
    }

    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let Some(element) = lhs.mut_element_type() else {
            return false;
        };
        Add::can_be_used(element.clone(), rhs.clone())
    }
}

impl Exec for AssignAdd {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        let lhs = lhs.into_mut().unwrap();
        let mut lhs = lhs.variable.write().unwrap();
        *lhs = lhs.clone() + rhs;
        Ok(lhs.clone())
    }
}

impl ReturnType for AssignAdd {
    fn return_type(&self) -> Type {
        self.rhs.return_type()
    }
}
