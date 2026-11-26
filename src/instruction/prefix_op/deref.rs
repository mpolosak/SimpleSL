use crate::{
    Interpreter,
    errors::{ErrorWithSpan, IncorrectUnaryOperatorOperand},
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithSpan, Recreate,
        local_variable::LocalVariables, parsing_context::ParsingContext,
    },
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type},
};

#[derive(Debug)]
pub struct Deref(Instruction);

impl Deref {
    pub fn create_instruction(
        instruction: InstructionWithSpan,
        parsing_context: &ParsingContext,
        errors: &mut Vec<ErrorWithSpan>,
    ) -> Instruction {
        let return_type = instruction.return_type();
        if return_type.mut_element_type().is_none() {
            let span = instruction.span;
            let ins = parsing_context.slice(instruction.span).into();
            let error = IncorrectUnaryOperatorOperand {
                ins,
                op: UnaryOperator::Indirection,
                expected: Type::Mut(Type::Any.into()),
                given: return_type,
            }
            .into();
            errors.push(ErrorWithSpan { error, span });
        }
        Self(instruction.instruction).into()
    }
}

impl BaseInstruction for Deref {}

impl Exec for Deref {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let value = self.0.exec(interpreter)?;
        Ok(value.into_mut().unwrap().variable.read().unwrap().clone())
    }
}

impl Recreate for Deref {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let ins = self.0.recreate(local_variables);
        Self(ins).into()
    }
}

impl ReturnType for Deref {
    fn return_type(&self) -> crate::variable::Type {
        self.0
            .return_type()
            .mut_element_type()
            .unwrap_or(Type::Never)
    }
}
