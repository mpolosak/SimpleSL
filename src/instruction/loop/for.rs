use crate::{
    self as simplesl, Error, Interpreter,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables,
        pattern::Pattern,
    },
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct For {
    pattern: Pattern,
    iter: Instruction,
    body: Instruction,
}

impl For {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pattern_pair = inner.next().unwrap();
        let iter = Instruction::new(inner.next().unwrap(), local_variables)?;
        let Some(iter_element) = iter.return_type().iter_element() else {
            return Err(Error::WrongType(
                "iterator".into(),
                var_type!(() -> (bool, any)),
            ));
        };
        let pattern = if pattern_pair.as_rule() == Rule::ident {
            Pattern::new_ident_pattern(pattern_pair.as_str().into(), iter_element.clone())
        } else {
            Pattern::create_instruction(pattern_pair.clone(), &iter_element)?
        };
        if !pattern.is_matched(&iter_element) {
            return Err(Error::SetPatternNotMatched {
                ins: "for loop element".into(),
                var_type: iter_element,
                pattern: pattern_pair.as_str().into(),
            });
        }
        let mut local_variables = local_variables.create_layer();
        local_variables.in_loop = true;
        pattern.insert_local_variables(&mut local_variables);
        let body = Instruction::new(inner.next().unwrap(), &mut local_variables)?;
        Ok(Self {
            pattern,
            iter,
            body,
        }
        .into())
    }
}

impl BaseInstruction for For {}

impl Exec for For {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iter = self.iter.exec(interpreter)?.into_function().unwrap();
        let mut interpreter = interpreter.create_layer();
        loop {
            let tuple = iter.exec(&[])?.into_tuple().unwrap();
            if let Variable::Bool(false) = tuple[0] {
                break;
            }
            let item = tuple[1].clone();
            if !self.pattern.matches_variable(&item) {
                continue;
            }
            self.pattern.insert_variables(&mut interpreter, item);
            self.body.exec(&mut interpreter)?;
        }
        Ok(Variable::Void)
    }
}

impl Recreate for For {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let iter = self.iter.recreate(local_variables);
        let mut local_variables = local_variables.create_layer();
        self.pattern.insert_local_variables(&mut local_variables);
        let body = self.body.recreate(&mut local_variables);
        Self {
            pattern: self.pattern.clone(),
            iter,
            body,
        }
        .into()
    }
}

impl ReturnType for For {
    fn return_type(&self) -> Type {
        Type::Void
    }
}

#[cfg(test)]
mod tests {
    use crate::{Code, Interpreter, variable::Variable};

    #[test]
    fn for_loop_destructures_tuple_pattern() {
        let script = "total := mut 0
                for (left, right) in [(1, 2), (3, 4)]~ {
                    total = *total + left + right
                }
                *total";
        let code = Code::parse(&Interpreter::without_stdlib(), script).unwrap();
        assert_eq!(code.exec().unwrap(), Variable::Int(10));
    }

    #[test]
    fn for_loop_keeps_identifier_binding() {
        let script = "total := mut 0
                for value in [1, 2, 3]~ {
                    total = *total + value
                }
                *total";
        let code = Code::parse(&Interpreter::without_stdlib(), script).unwrap();
        assert_eq!(code.exec().unwrap(), Variable::Int(6));
    }

    #[test]
    fn for_loop_skips_items_that_do_not_match_pattern() {
        let script = "total := mut 0
            for [value] in [[], [2]]~ {
                total = *total + value
            }
            *total";
        let code = Code::parse(&Interpreter::without_stdlib(), script).unwrap();
        assert_eq!(code.exec().unwrap(), Variable::Int(2));
    }
}
