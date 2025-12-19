use crate::{
    instruction::{
        local_variable::LocalVariables, pattern::Pattern, Exec, ExecResult, Instruction, InstructionWithStr, Recreate
    }, interpreter::Interpreter, variable::{ReturnType, Type, Typed, Variable}, Error, ExecError
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct SetIfElse {
    pattern: Pattern,
    expression: InstructionWithStr,
    if_match: InstructionWithStr,
    pub else_instruction: InstructionWithStr,
}

impl SetIfElse {
    pub fn create(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let pattern_pair = inner.next().unwrap();
        let pair = inner.next().unwrap();
        let expression = InstructionWithStr::new(pair, local_variables)?;
        let pattern = Pattern::create_instruction(pattern_pair, local_variables, &expression.return_type());
        let pair = inner.next().unwrap();
        let if_match = {
            let mut local_variables = local_variables.create_layer();
            pattern.insert_local_variables(&mut local_variables);
            InstructionWithStr::new(pair, &mut local_variables)?
        };
        let else_instruction = inner
            .next()
            .map(|pair| InstructionWithStr::new(pair, local_variables))
            .unwrap_or(Ok(Variable::Void.into()))?;
        Ok(Self {
            pattern,
            expression,
            if_match,
            else_instruction,
        })
    }

    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        Ok(Self::create(pair, local_variables)?.into())
    }
}

impl Exec for SetIfElse {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let expression_result = self.expression.exec(interpreter)?;
        let result_type = expression_result.as_type();
        if !self.pattern.is_matched(&result_type) {
            return self.else_instruction.exec(interpreter);
        }
        let mut interpreter = interpreter.create_layer();
        self.pattern.insert_variables(&mut interpreter, expression_result);
        self.if_match.exec(&mut interpreter)
    }
}

impl Recreate for SetIfElse {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let expression = self.expression.recreate(local_variables)?;
        let if_match = {
            let mut local_variables = local_variables.create_layer();
            self.pattern.insert_local_variables(&mut local_variables);
            self.if_match.recreate(&mut local_variables)?
        };
        let else_instruction = self.else_instruction.recreate(local_variables)?;
        Ok(Self {
            pattern: self.pattern.clone(),
            expression,
            if_match,
            else_instruction,
        }
        .into())
    }
}

impl ReturnType for SetIfElse {
    fn return_type(&self) -> Type {
        let true_return_type = self.if_match.return_type();
        let false_return_type = self.else_instruction.return_type();
        true_return_type | false_return_type
    }
}
