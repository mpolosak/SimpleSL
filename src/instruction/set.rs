use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
};
use crate::{ instruction::{function::Function, pattern::{destruct_pattern::DestructPattern, Pattern}}, interpreter::Interpreter, variable::{ReturnType, Type}, Error, ExecError
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Set {
    pub pattern: Pattern,
    pub instruction: InstructionWithStr,
}

impl Set {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pattern_pair = inner.next().unwrap();
        let pair = inner.next().unwrap();
        let ins = pair.as_str().into();
        if pair.as_rule() == Rule::function {
            return Self::create_function_declaration(pattern_pair, pair, local_variables);
        }
        let instruction = InstructionWithStr::new(pair, local_variables)?;
        let var_type = instruction.return_type();
        let pattern_str = pattern_pair.as_str().into();
        let pattern = Pattern::create_instruction(pattern_pair, local_variables, &var_type)?;
        if !pattern.is_matched(&var_type) {
            return Err(Error::SetPatternNotMatched { ins, var_type, pattern: pattern_str })
        }
        pattern.insert_local_variables(local_variables);
        Ok(Self{ pattern, instruction }.into())
    }

    fn create_function_declaration(pattern_pair: Pair<Rule>, function_pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Instruction, Error> { 
        let pattern_str = pattern_pair.as_str().into();
        let mut pattern = Pattern::create_instruction(pattern_pair, local_variables, &Type::Any)?;
        let ident = if let DestructPattern::Ident(ident) = &pattern.destruct_pattern { Some(ident) } else {
            None
        };
        let ins = function_pair.as_str().into();
        let function = Function::create_instruction(function_pair, local_variables, ident.cloned())?;
        let var_type = function.return_type();
        if !pattern.is_matched(&var_type) {
            return Err(Error::SetPatternNotMatched { ins, var_type, pattern: pattern_str })
        }
        pattern.var_type = pattern.var_type.conjoin(&function.return_type());
        pattern.insert_local_variables(local_variables);
        let instruction = InstructionWithStr { instruction: function, str: ins };
        Ok(Self{ pattern, instruction }.into())
    }
}

impl Exec for Set {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let result = self.instruction.exec(interpreter)?;
        self.pattern.insert_variables(interpreter, result.clone());
        Ok(result)
    }
}

impl Recreate for Set {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        self.pattern.insert_local_variables(local_variables);
        Ok(Self {
            pattern: self.pattern.clone(),
            instruction,
        }
        .into())
    }
}

impl ReturnType for Set {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}
