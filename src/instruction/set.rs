use super::{Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables};
use crate::{
    Error,
    errors::{ErrorWithRange, SetPatternNotMatched, WrongType},
    instruction::{
        BaseInstruction,
        function::Function,
        parsing_context::ParsingContext,
        pattern::{Pattern, destruct_pattern::DestructPattern},
    },
    interpreter::Interpreter,
    range_from_pest_span,
    variable::{ReturnType, Type},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug)]
pub struct Set {
    pub pattern: Pattern,
    pub instruction: Instruction,
}

impl Set {
    pub fn create(
        pair: Pair<Rule>,
        parsing_context: &mut ParsingContext,
        errors: &mut Vec<ErrorWithRange>,
    ) -> Self {
        let is_declaration = pair.as_rule() == Rule::set;
        let range = range_from_pest_span(&pair.as_span());
        let (set, error) = Self::new(pair, parsing_context, errors);
        if is_declaration && matches!(&set.pattern.destruct_pattern, DestructPattern::Array { .. })
        {
            let error = Error::ArrayPatternRequiresElse;
            errors.push(ErrorWithRange {
                error,
                range: range.clone(),
            });
        }
        if !set.pattern.is_matched(&set.instruction.return_type()) {
            errors.push(ErrorWithRange { error, range });
        }
        set
    }

    fn new(
        pair: Pair<Rule>,
        parsing_context: &mut ParsingContext,
        errors: &mut Vec<ErrorWithRange>,
    ) -> (Self, Error) {
        let mut inner = pair.into_inner();
        let pattern_pair = inner.next().unwrap();
        let pair = inner.next().unwrap();
        let ins = pair.as_str().into();
        if pair.as_rule() == Rule::function {
            return Self::create_function_declaration(pattern_pair, pair, parsing_context, errors);
        }
        let instruction = Instruction::new(pair, parsing_context, errors);
        let var_type = instruction.return_type();
        let pattern_str = pattern_pair.as_str().into();
        let pattern = Pattern::create_instruction(pattern_pair, &var_type, errors);
        pattern.insert_local_variables(&mut parsing_context.local_variables);
        (
            Self {
                pattern,
                instruction,
            },
            SetPatternNotMatched {
                ins,
                var_type,
                pattern: pattern_str,
            }
            .into(),
        )
    }

    fn create_function_declaration(
        pattern_pair: Pair<Rule>,
        function_pair: Pair<Rule>,
        parsing_context: &mut ParsingContext,
        errors: &mut Vec<ErrorWithRange>,
    ) -> (Self, Error) {
        let pattern_str = pattern_pair.as_str().into();
        let mut pattern = Pattern::create_instruction(pattern_pair, &Type::Any, errors);
        let ident = if let DestructPattern::Ident(ident) = &pattern.destruct_pattern {
            Some(ident)
        } else {
            None
        };
        let ins: Arc<str> = function_pair.as_str().into();
        let instruction =
            Function::create_instruction(function_pair, parsing_context, ident.cloned(), errors);
        let var_type = instruction.return_type();
        pattern.var_type = pattern.var_type.conjoin(&instruction.return_type());
        pattern.insert_local_variables(&mut parsing_context.local_variables);
        (
            Self {
                pattern,
                instruction,
            },
            SetPatternNotMatched {
                ins,
                var_type,
                pattern: pattern_str,
            }
            .into(),
        )
    }

    pub fn inner_recreate(&self, local_variables: &mut LocalVariables) -> Self {
        let instruction = self.instruction.recreate(local_variables);
        self.pattern.insert_local_variables(local_variables);
        Self {
            pattern: self.pattern.clone(),
            instruction,
        }
    }

    pub fn new_ident(ident: Arc<str>, instruction: Instruction) -> Set {
        Set {
            pattern: Pattern::new_ident_pattern(ident, instruction.return_type()),
            instruction,
        }
    }
}

impl BaseInstruction for Set {}

impl Exec for Set {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let result = self.instruction.exec(interpreter)?;
        self.pattern.insert_variables(interpreter, result.clone());
        Ok(result)
    }
}

impl Recreate for Set {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        self.inner_recreate(local_variables).into()
    }
}

impl ReturnType for Set {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}

#[derive(Debug)]
pub struct SetElse {
    pattern: Pattern,
    instruction: Instruction,
    else_instruction: Instruction,
}

impl SetElse {
    pub fn create(
        pair: Pair<Rule>,
        parsing_context: &mut ParsingContext,
        errors: &mut Vec<ErrorWithRange>,
    ) -> Instruction {
        let mut inner = pair.into_inner();
        let pattern_pair = inner.next().unwrap();
        let instruction = Instruction::new(inner.next().unwrap(), parsing_context, errors);
        let pattern = Pattern::create_instruction(pattern_pair, &instruction.return_type(), errors);
        let else_pair = inner.next().unwrap();
        let span = else_pair.as_span();
        let else_instruction = Instruction::new(else_pair, parsing_context, errors);
        if else_instruction.return_type() != Type::Never {
            let error = WrongType {
                ident: "else branch".into(),
                expected_type: Type::Never,
            }
            .into();
            let span = range_from_pest_span(&span);
            errors.push(ErrorWithRange { error, range: span });
        }
        pattern.insert_local_variables(&mut parsing_context.local_variables);
        Self {
            pattern,
            instruction,
            else_instruction,
        }
        .into()
    }
}

impl BaseInstruction for SetElse {}

impl Exec for SetElse {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let value = self.instruction.exec(interpreter)?;
        if self.pattern.matches_variable(&value) {
            self.pattern.insert_variables(interpreter, value.clone());
            return Ok(value);
        }
        match self.else_instruction.exec(interpreter) {
            Err(stop) => Err(stop),
            Ok(_) => unreachable!("let-else branch must diverge"),
        }
    }
}

impl Recreate for SetElse {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.instruction.recreate(local_variables);
        let else_instruction = self.else_instruction.recreate(local_variables);
        self.pattern.insert_local_variables(local_variables);
        Self {
            pattern: self.pattern.clone(),
            instruction,
            else_instruction,
        }
        .into()
    }
}

impl ReturnType for SetElse {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}

#[derive(Debug)]
pub struct ConditionSet(Set);

impl ConditionSet {
    pub fn create(
        pair: Pair<Rule>,
        parsing_context: &mut ParsingContext,
        errors: &mut Vec<ErrorWithRange>,
    ) -> Self {
        ConditionSet(Set::new(pair, parsing_context, errors).0)
    }
}

impl BaseInstruction for ConditionSet {}

impl Exec for ConditionSet {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let result = self.0.instruction.exec(interpreter)?;
        if !self.0.pattern.matches_variable(&result) {
            return Ok(false.into());
        }
        self.0.pattern.insert_variables(interpreter, result.clone());
        Ok(true.into())
    }
}

impl Recreate for ConditionSet {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        Self(self.0.inner_recreate(local_variables)).into()
    }
}

impl ReturnType for ConditionSet {
    fn return_type(&self) -> Type {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, Interpreter,
        errors::{SelfContradictoryPattern, SetPatternNotMatched, WrongCondition, WrongType},
        test::parse_unwrap_first_error,
        variable::{Array, Type, Variable},
    };
    use simplesl_macros::var_type;

    #[test]
    fn destructures_when_pattern_matches() {
        assert_eq!(
            parse_and_exec(
                "sum_pair := (value: int | (int, int)) -> int {
                    (left, right): (int, int) = value else { return -1 }
                    return left + right
                }
                sum_pair((2, 3))"
            ),
            Variable::Int(5)
        );
    }

    #[test]
    fn executes_else_when_pattern_does_not_match() {
        assert_eq!(
            parse_and_exec(
                "sum_pair := (value: int | (int, int)) -> int {
                    (left, right): (int, int) = value else { return -1 }
                    return left + right
                }
                sum_pair(4)"
            ),
            Variable::Int(-1)
        );
    }

    #[test]
    fn rejects_non_diverging_else() {
        assert_eq!(
            parse_unwrap_first_error(
                "sum_pair := (value: int | (int, int)) -> int {
                    (left, right): (int, int) = value else { 0 }
                    return left + right
                }
                sum_pair(4)"
            ),
            WrongType {
                ident: "else branch".into(),
                expected_type: Type::Never
            }
            .into()
        );
    }

    #[test]
    fn else_can_continue_loop() {
        assert_eq!(
            parse_and_exec(
                "loop {
                    (left, right): (int, int) = (1, 2) else { break }
                    break
                }"
            ),
            Variable::none()
        );
    }

    #[test]
    fn destructures_struct_fields_by_name_and_alias() {
        assert_eq!(
            parse_and_exec(
                "person := struct{name := \"Ada\", age := 37}
                struct{name: first_name, age} := person
                match person {
                    struct{name: matched_name} => matched_name,
                    => \"unknown\",
                }"
            ),
            Variable::String("Ada".into())
        );
        assert_eq!(
            parse_and_exec(
                "get_name := (value: struct{name: int} | struct{age: int}) -> int {
                    struct{name: name} := value else { return -1 }
                    return name
                }
                get_name(struct{age := 3})"
            ),
            Variable::Int(-1)
        );
    }

    #[test]
    fn destructures_array_with_else_branch() {
        assert_eq!(
            parse_and_exec(
                "sum_pair := (values: [int]) -> int {
                    [first, second] := values else { return -1 }
                    return first + second
                }
                sum_pair([2, 3])"
            ),
            Variable::Int(5)
        );
    }

    #[test]
    fn executes_else_for_array_with_wrong_length() {
        assert_eq!(
            parse_and_exec(
                "sum_pair := (values: [int]) -> int {
                    [first, second] := values else { return -1 }
                    return first + second
                }
                sum_pair([2])"
            ),
            Variable::Int(-1)
        );
    }

    #[test]
    fn captures_array_rest_after_prefix() {
        assert_eq!(
            parse_and_exec(
                "tail := (values: [int]) -> [int] {
                    [first, ..rest] := values else { return [] }
                    return rest
                }
                tail([1, 2, 3])"
            ),
            Variable::from([2.into(), 3.into()])
        );
    }

    #[test]
    fn captures_empty_array_rest() {
        assert_eq!(
            parse_and_exec(
                "tail := (values: [int]) -> [int] {
                    [first, ..rest] := values else { return [] }
                    return rest
                }
                tail([1])"
            ),
            Array::new_with_type(Type::Int, Vec::<Variable>::new().into()).into()
        );
    }

    #[test]
    fn rest_pattern_requires_minimum_prefix_length() {
        assert_eq!(
            parse_and_exec(
                "tail := (values: [int]) -> [int] {
                    [first, ..rest] := values else { return [] }
                    return rest
                }
                tail([])"
            ),
            Variable::from([])
        );
    }

    #[test]
    fn rejects_array_destructuring_without_else_branch() {
        assert_eq!(
            parse_unwrap_first_error("[first, second] := [2, 3]"),
            Error::ArrayPatternRequiresElse
        );
    }

    #[test]
    fn array_pattern_in_match_checks_length() {
        assert_eq!(
            parse_unwrap_first_error("match [2] { [item] => item, }"),
            Error::MatchNotCovered
        );
        assert_eq!(
            parse_and_exec(
                "match [2] {
                    [first, second] => first + second,
                    => -1,
                }"
            ),
            Variable::Int(-1)
        );
        assert_eq!(
            parse_and_exec(
                "match [2, 3] {
                    [first, second] => first + second,
                    => -1,
                }"
            ),
            Variable::Int(5)
        );
        assert_eq!(
            parse_and_exec(
                "match [1, 2, 3] {
                    [first, ..rest] => rest,
                    => [],
                }"
            ),
            Variable::from([2.into(), 3.into()])
        );
    }

    #[test]
    fn reject_self_contradictory_pattern() {
        assert_eq!(
            parse_unwrap_first_error("(a, b): any = (1, 2)"),
            SelfContradictoryPattern {
                pattern: "(a, b): any".into()
            }
            .into()
        )
    }

    #[test]
    fn struct_destructing_rejects_multi_type_if_only_one_variant_has_the_requested_field() {
        assert_eq!(
            parse_unwrap_first_error(
                "f := (x: bool) -> struct{a: int} | struct{b: int} {
                if x { return struct{a := 1} } else { return struct{b := 2} }
            }
            struct{a}: struct{a: int} = f(false)"
            ),
            SetPatternNotMatched {
                ins: "f(false)".into(),
                var_type: var_type!(struct{a: int} | struct{b: int}),
                pattern: "struct{a}: struct{a: int}".into()
            }
            .into()
        )
    }

    #[test]
    fn match_guard_can_use_pattern_binding() {
        assert_eq!(
            parse_and_exec(
                "match 7 {
                    value: int if value > 5 => value,
                    => -1,
                }"
            ),
            Variable::Int(7)
        );
    }

    #[test]
    fn match_guard_falls_through_when_false() {
        assert_eq!(
            parse_and_exec(
                "match 3 {
                    value: int if value > 5 => value,
                    => -1,
                }"
            ),
            Variable::Int(-1)
        );
    }

    #[test]
    fn match_guard_supports_condition_chains_and_bindings() {
        assert_eq!(
            parse_and_exec(
                "match 7 {
                    value: int if positive: bool = value > 0; positive && value < 10 => value,
                    => -1,
                }"
            ),
            Variable::Int(7)
        );
    }

    #[test]
    fn match_guard_condition_chain_short_circuits() {
        assert_eq!(
            parse_and_exec(
                "match 0 {
                    value: int if positive: bool = value > 0; positive && 1 / value > 0 => value,
                    => -1,
                }"
            ),
            Variable::Int(-1)
        );
    }

    #[test]
    fn guarded_match_arm_does_not_cover_type() {
        assert_eq!(
            parse_unwrap_first_error("match 1 { value: int if true => value, }"),
            Error::MatchNotCovered
        );
    }

    #[test]
    fn match_guard_must_be_boolean() {
        assert_eq!(
            parse_unwrap_first_error("match 1 { value: int if value => value, => 0, }"),
            WrongCondition {
                ins: "value".into(),
                var_type: Type::Int
            }
            .into()
        );
    }

    fn parse_and_exec(script: &str) -> Variable {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
            .unwrap()
    }
}
