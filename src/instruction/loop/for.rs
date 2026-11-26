use crate::{
    self as simplesl, Interpreter,
    errors::{ErrorWithSpan, SetPatternNotMatched, WrongType},
    instruction::{
        BaseInstruction, Exec, ExecResult, ExecStop, Instruction, Recreate,
        local_variable::LocalVariables, parsing_context::ParsingContext, pattern::Pattern,
    },
    source_span_from_pest_span,
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
        parsing_context: &mut ParsingContext,
        errors: &mut Vec<ErrorWithSpan>,
    ) -> Instruction {
        let mut inner = pair.into_inner();
        let pattern_pair = inner.next().unwrap();
        let iter_pair = inner.next().unwrap();
        let iter_span = iter_pair.as_span();
        let iter = Instruction::new(iter_pair, parsing_context, errors);
        let iter_element = iter.return_type().iter_element().unwrap_or_else(|| {
            let error = WrongType {
                ident: "iterator".into(),
                expected_type: var_type!(() -> (#Some(any)|#None)),
            }
            .into();
            let span = source_span_from_pest_span(&iter_span);
            errors.push(ErrorWithSpan { error, span });
            Type::Never
        });
        let span = pattern_pair.as_span();
        let pattern = if pattern_pair.as_rule() == Rule::ident {
            Pattern::new_ident_pattern(pattern_pair.as_str().into(), iter_element.clone())
        } else {
            Pattern::create_instruction(pattern_pair.clone(), &iter_element, errors)
        };
        if !pattern.is_matched(&iter_element) {
            let span = source_span_from_pest_span(&span);
            let error = SetPatternNotMatched {
                ins: "for loop element".into(),
                var_type: iter_element,
                pattern: pattern_pair.as_str().into(),
            }
            .into();
            errors.push(ErrorWithSpan { error, span });
        }
        let mut parsing_context = parsing_context.create_layer();
        parsing_context.local_variables.in_loop = true;
        pattern.insert_local_variables(&mut parsing_context.local_variables);
        let body = Instruction::new(inner.next().unwrap(), &mut parsing_context, errors);
        Self {
            pattern,
            iter,
            body,
        }
        .into()
    }
}

impl BaseInstruction for For {}

impl Exec for For {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iter = self.iter.exec(interpreter)?.into_function().unwrap();
        let mut interpreter = interpreter.create_layer();
        loop {
            let tagged = iter.exec(&[])?.into_tagged().unwrap();
            if tagged.tag.as_ref() == "None" {
                break;
            }
            let item = tagged.payload.clone().unwrap();
            if !self.pattern.matches_variable(&item) {
                continue;
            }
            self.pattern.insert_variables(&mut interpreter, item);
            match self.body.exec(&mut interpreter) {
                Ok(_) | Err(ExecStop::Continue) => (),
                Err(ExecStop::Break) => break,
                stop => return stop,
            }
        }
        Ok(Variable::none())
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
        Type::none()
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

    #[test]
    fn for_loop_break_exits_current_loop() {
        let script = "total := mut 0
            for value in [1, 2, 3]~ {
                if value == 2 {
                    break
                }
                total = *total + value
            }
            *total";
        let code = Code::parse(&Interpreter::without_stdlib(), script).unwrap();
        assert_eq!(code.exec().unwrap(), Variable::Int(1));
    }

    #[test]
    fn for_loop_continue_skips_to_next_item() {
        let script = "total := mut 0
            for value in [1, 2, 3]~ {
                if value == 2 {
                    continue
                }
                total = *total + value
            }
            *total";
        let code = Code::parse(&Interpreter::without_stdlib(), script).unwrap();
        assert_eq!(code.exec().unwrap(), Variable::Int(4));
    }

    #[test]
    fn nested_for_loop_break_only_exits_inner_loop() {
        let script = "total := mut 0
            for outer in [1, 2]~ {
                for inner in [1, 2, 3]~ {
                    if inner == 2 {
                        break
                    }
                    total = *total + outer
                }
            }
            *total";
        let code = Code::parse(&Interpreter::without_stdlib(), script).unwrap();
        assert_eq!(code.exec().unwrap(), Variable::Int(3));
    }
}
