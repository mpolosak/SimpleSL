use super::{ExecResult, Instruction, InstructionWithSpan, local_variable::LocalVariables};
use crate::{
    Interpreter,
    errors::{CannotIndexInto, CannotIndexWith, ErrorWithSpan},
    instruction::{BaseInstruction, Exec, Recreate, parsing_context::ParsingContext},
    stdlib::operators::at,
    variable::{ReturnType, Type},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
struct At {
    instruction: Instruction,
    index: Instruction,
}

pub fn create(
    instruction: InstructionWithSpan,
    index: Pair<Rule>,
    parsing_context: &ParsingContext,
    errors: &mut Vec<ErrorWithSpan>,
) -> Instruction {
    let pair = index.into_inner().next().unwrap();
    let index = InstructionWithSpan::new_expression(pair, parsing_context, errors);
    let instruction_return_type = instruction.return_type();
    if !instruction_return_type.can_be_indexed() {
        let error = CannotIndexInto {
            var_type: instruction_return_type,
        }
        .into();
        let span = instruction.span;
        errors.push(ErrorWithSpan { error, span });
    }
    if !index.return_type().matches(&Type::Int) {
        let span = index.span;
        let ins = parsing_context.slice(span).into();
        let error = CannotIndexWith { ins }.into();
        errors.push(ErrorWithSpan { error, span });
    }
    At {
        instruction: instruction.instruction,
        index: index.instruction,
    }
    .into()
}

impl BaseInstruction for At {}

impl Exec for At {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let variable = self.instruction.exec(interpreter)?;
        let index = self.index.exec(interpreter)?.into_int().unwrap();
        Ok(at(&variable, index)?)
    }
}

impl Recreate for At {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.instruction.recreate(local_variables);
        let index = self.index.recreate(local_variables);
        Self { instruction, index }.into()
    }
}

impl ReturnType for At {
    fn return_type(&self) -> Type {
        self.instruction
            .return_type()
            .index_result()
            .unwrap_or(Type::Never)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, ExecError, Interpreter,
        errors::{CannotIndexInto, CannotIndexWith},
        variable::Variable,
    };
    use simplesl_macros::var_type;

    #[test]
    fn index_array() {
        assert_eq!(
            parse_and_exec("[45, 5.5, false][2]"),
            Ok(Variable::Bool(false))
        )
    }

    #[test]
    fn index_array_negative_index() {
        assert_eq!(
            parse_and_exec("[45, 5.5, false][-1]"),
            Ok(Variable::Bool(false))
        )
    }

    #[test]
    fn index_string() {
        assert_eq!(
            parse_and_exec(r#""żaba🐸"[2]"#),
            Ok(Variable::String("b".into()))
        )
    }

    #[test]
    fn index_string_negative_index() {
        assert_eq!(
            parse_and_exec(r#""żaba🐸"[-1]"#),
            Ok(Variable::String("🐸".into()))
        )
    }

    #[test]
    fn index_array_out_of_bounds() {
        assert_eq!(
            parse_and_exec("[45, 5.5, false][3]"),
            Err(ExecError::IndexOutOfBounds)
        )
    }

    #[test]
    fn index_array_negative_index_out_of_bounds() {
        assert_eq!(
            parse_and_exec("[45, 5.5, false][-4]"),
            Err(ExecError::IndexOutOfBounds)
        )
    }

    #[test]
    fn index_string_out_of_bounds() {
        assert_eq!(
            parse_and_exec(r#""żaba🐸"[5]"#),
            Err(ExecError::IndexOutOfBounds)
        )
    }

    #[test]
    fn index_string_negative_index_out_of_bounds() {
        assert_eq!(
            parse_and_exec(r#""żaba🐸"[-6]"#),
            Err(ExecError::IndexOutOfBounds)
        )
    }

    #[test]
    fn at_rejects_indexing_with_float() {
        assert_eq!(
            parse_unwrap_first_error("[45, 5.5, false][2.5]"),
            CannotIndexWith { ins: "2.5".into() }.into()
        )
    }

    #[test]
    fn at_rejects_indexing_into_tuple() {
        assert_eq!(
            parse_unwrap_first_error("(45, 5.5, false)[2]"),
            CannotIndexInto {
                var_type: var_type!((int, float, bool))
            }
            .into()
        )
    }

    #[test]
    fn at_rejects_indexing_into_bool_with_none() {
        assert_eq!(
            parse_unwrap_errors("true[#None]"),
            vec![
                CannotIndexInto {
                    var_type: var_type!(bool)
                }
                .into(),
                CannotIndexWith {
                    ins: "#None".into()
                }
                .into()
            ]
        )
    }

    #[test]
    fn at() {
        assert_eq!(
            parse_unwrap_errors("(45, 4.5)[0][0]"),
            vec![
                CannotIndexInto {
                    var_type: var_type!((int, float))
                }
                .into()
            ]
        );
        assert_eq!(
            parse_unwrap_errors("(45, 4.5)[0][0.0]"),
            vec![
                CannotIndexInto {
                    var_type: var_type!((int, float))
                }
                .into(),
                CannotIndexWith { ins: "0.0".into() }.into()
            ]
        )
    }

    fn parse_and_exec(script: &str) -> Result<Variable, ExecError> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
    }

    fn parse_unwrap_first_error(script: &str) -> Error {
        parse_unwrap_errors(script).swap_remove(0)
    }

    fn parse_unwrap_errors(script: &str) -> Vec<Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap_err()
            .errors
            .into_iter()
            .map(|e| e.error)
            .collect()
    }
}
