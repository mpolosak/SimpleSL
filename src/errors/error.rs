use crate::{BinOperator, function::Param, unary_operator::UnaryOperator, variable::Type};
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use derive_more::{Display, From};
use itertools::join;
use match_any::match_any;
use pest::error::InputLocation;
use simplesl_parser::Rule;
use std::{
    fmt::{self},
    ops::Range,
    sync::Arc,
};

#[derive(Debug, Display, PartialEq)]
#[display("Errors in {} {{\n{}\n}}", source.name(), join(errors, "\n"))]
pub struct ErrorsWithSource {
    pub source: Source,
    pub errors: Vec<ErrorWithRange>,
}

impl std::error::Error for ErrorsWithSource {}

impl ErrorsWithSource {
    pub fn report(&self) {
        let file = SimpleFile::new(self.source.name.as_ref(), self.source.content.as_ref());
        for error in self.errors.iter() {
            error.report(&file);
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Source {
    name: String,
    content: String,
}

impl Source {
    pub fn new(name: impl Into<String>, content: String) -> Self {
        Self {
            name: name.into(),
            content,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Display, PartialEq)]
#[display("{error} on {}:{}", range.start, range.end)]
pub struct ErrorWithRange {
    pub error: Error,
    pub range: Range<usize>,
}

impl ErrorWithRange {
    fn report(&self, file: &SimpleFile<&str, &str>) {
        if let Error::ErrorsWithSource(error) = &self.error {
            error.report();
            return;
        }

        let diagnostic = Diagnostic::error().with_message("error").with_label(
            Label::primary((), self.range.clone()).with_message(self.error.to_string()),
        );

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        term::emit_to_write_style(&mut writer.lock(), &config, file, &diagnostic).unwrap();
    }
}

impl std::error::Error for ErrorWithRange {}

impl From<pest::error::Error<Rule>> for ErrorWithRange {
    fn from(value: pest::error::Error<Rule>) -> Self {
        let range = range_from_input_location(value.location);
        let error = value.variant.into();
        Self { error, range }
    }
}

pub(crate) fn range_from_input_location(loc: InputLocation) -> Range<usize> {
    match loc {
        InputLocation::Pos(start) => start..start,
        InputLocation::Span((start, end)) => start..end,
    }
}

#[derive(Debug, From, Display)]
pub enum Error {
    #[display("Break outside loop")]
    BreakOutsideLoop,
    #[from]
    CannotDetermineParams(CannotDetermineParams),
    #[from]
    CannotReduce(CannotReduce),
    #[display("Continue outside loop")]
    ContinueOutsideLoop,
    #[from]
    VariableDoesntExist(VariableDoesntExist),
    #[from]
    WrongType(WrongType),
    #[from]
    WrongNumberOfArguments(WrongNumberOfArguments),
    #[from]
    TupleIndexTooBig(TupleIndexTooBig),
    #[from]
    CannotBeParsed(CannotBeParsed),
    #[from]
    CannotIndexInto(CannotIndexInto),
    #[from]
    CannotTupleAccess(CannotTupleAccess),
    #[from]
    CannotFieldAccess(CannotFieldAccess),
    #[from]
    CannotIndexWith(CannotIndexWith),
    #[from]
    CannotSlice(CannotSlice),
    #[display("All posible values must be covered in match")]
    MatchNotCovered,
    #[from]
    IO(std::io::Error),
    #[from]
    Parsing(pest::error::ErrorVariant<Rule>),
    #[from]
    IntegerOverflow(IntegerOverflow),
    #[from]
    IncorrectUnaryOperatorOperand(IncorrectUnaryOperatorOperand),
    #[from]
    CannotUnescapeString(unescaper::Error),
    #[from]
    CannotDo2(CannotDo2),
    #[from]
    WrongReturn(WrongReturn),
    #[display("Return statement can only be used inside of function body")]
    ReturnOutsideFunction,
    #[from]
    MissingReturn(MissingReturn),
    #[from]
    NoField(NoField),
    #[from]
    SelfContradictoryPattern(SelfContradictoryPattern),
    #[display("Array patterns require an else branch")]
    ArrayPatternRequiresElse,
    #[from]
    SetPatternNotMatched(SetPatternNotMatched),
    #[from]
    WrongLengthType(WrongLengthType),
    #[from]
    NotAFunction(NotAFunction),
    #[from]
    WrongArgument(WrongArgument),
    #[from]
    WrongCondition(WrongCondition),
    #[from]
    WrongInitialization(WrongInitialization),
    #[from]
    ErrorsWithSource(ErrorsWithSource),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match_any! { (self, other),
            (Self::VariableDoesntExist(l0), Self::VariableDoesntExist(r0))
            | (Self::CannotBeParsed(l0), Self::CannotBeParsed(r0))
            | (Self::CannotIndexInto(l0), Self::CannotIndexInto(r0))
            | (Self::CannotIndexWith(l0), Self::CannotIndexWith(r0))
            | (Self::Parsing(l0), Self::Parsing(r0))
            | (Self::IntegerOverflow(l0), Self::IntegerOverflow(r0))
            | (Self::NotAFunction(l0), Self::NotAFunction(r0))
            | (Self::CannotDetermineParams(l0), Self::CannotDetermineParams(r0))
            | (Self::CannotReduce(l0), Self::CannotReduce(r0))
            | (Self::SelfContradictoryPattern(l0), Self::SelfContradictoryPattern(r0))
            | (Self::WrongType(l0), Self::WrongType(r0))
            | (Self::WrongNumberOfArguments(l0), Self::WrongNumberOfArguments(r0))
            | (Self::TupleIndexTooBig(l0), Self::TupleIndexTooBig(r0))
            | (Self::IncorrectUnaryOperatorOperand(l0), Self::IncorrectUnaryOperatorOperand(r0))
            | (Self::CannotDo2(l0), Self::CannotDo2(r0))
            | (Self::WrongReturn(l0), Self::WrongReturn(r0))
            | (Self::MissingReturn(l0), Self::MissingReturn(r0))
            | (Self::NoField(l0), Self::NoField(r0))
            | (Self::SetPatternNotMatched(l0), Self::SetPatternNotMatched(r0))
            | (Self::WrongArgument(l0), Self::WrongArgument(r0))
            | (Self::WrongInitialization(l0), Self::WrongInitialization(r0))
            | (Self::WrongCondition(l0), Self::WrongCondition(r0))
            | (Self::CannotTupleAccess(l0), Self::CannotTupleAccess(r0))
            | (Self::CannotFieldAccess(l0), Self::CannotFieldAccess(r0))
            | (Self::CannotSlice(l0), Self::CannotSlice(r0))
            | (Self::ErrorsWithSource(l0), Self::ErrorsWithSource(r0))
                => l0 == r0,
            (Self::IO(l0), Self::IO(r0))
            | (Self::CannotUnescapeString(l0), Self::CannotUnescapeString(r0)) => {
                l0.to_string() == r0.to_string()
            },
            _ => core::mem::discriminant(self) == core::mem::discriminant(other)
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("Cannot determine params of function {ident}")]
pub struct CannotDetermineParams {
    pub ident: Arc<str>,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("Cannot reduce {str}. It is not an array")]
pub struct CannotReduce {
    pub str: Arc<str>,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("{ident} doesn't exist")]
pub struct VariableDoesntExist {
    pub ident: Arc<str>,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("{ident} should be {expected_type}")]
pub struct WrongType {
    pub ident: Arc<str>,
    pub expected_type: Type,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WrongNumberOfArguments {
    pub function: Arc<str>,
    pub expected_arguments: usize,
}

impl fmt::Display for WrongNumberOfArguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.expected_arguments == 0 {
            write!(f, "{} requires no arguments but some passed", self.function)
        } else {
            write!(
                f,
                "{} requires {} args",
                self.function, self.expected_arguments
            )
        }
    }
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("Cannot get element {index} of tuple {ins}. Tuple has len of {len}")]
pub struct TupleIndexTooBig {
    pub index: usize,
    pub ins: Arc<str>,
    pub len: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IncorrectUnaryOperatorOperand {
    pub ins: Arc<str>,
    pub op: UnaryOperator,
    pub expected: Type,
    pub given: Type,
}

impl fmt::Display for IncorrectUnaryOperatorOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.op.is_prefix() {
            write!(
                f,
                "Cannot {} {}. Operand need to be {} but {} which is {} was given",
                self.op, self.ins, self.expected, self.ins, self.given
            )
        } else {
            write!(
                f,
                "Cannot {} {}. Operand need to be {} but {} wh#[derive(Debug, PartialEq, Eq, Display)]ich is {} was given",
                self.ins, self.op, self.expected, self.ins, self.given
            )
        }
    }
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("Cannot do {lhs} {op} {rhs}")]
pub struct CannotDo2 {
    pub lhs: Type,
    pub op: BinOperator,
    pub rhs: Type,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display(
                    "Cannot return {returned} from function{}\n\
                    Function{0} declared to return {function_return_type}",
                    function_name
                        .as_deref()
                        .map(|value| format!(" {value}"))
                        .unwrap_or("".into()))]
pub struct WrongReturn {
    pub function_name: Option<Arc<str>>,
    pub function_return_type: Type,
    pub returned: Type,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("Function{} declared to return {return_type} may exit without returning any value\n\
                add return statement at the end of the function or change return type of the function to include ()",
                function_name
                    .as_deref()
                    .map(|value| format!(" {value}"))
                    .unwrap_or("".into()))]
pub struct MissingReturn {
    pub function_name: Option<Arc<str>>,
    pub return_type: Type,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("{struct_ident} has no field '{field_ident}'. Type of {struct_ident} is {struct_type}")]
pub struct NoField {
    pub struct_ident: Arc<str>,
    pub field_ident: Arc<str>,
    pub struct_type: Type,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("{ins} of type {var_type} doesn't match pattern {pattern}")]
pub struct SetPatternNotMatched {
    pub ins: Arc<str>,
    pub var_type: Type,
    pub pattern: Arc<str>,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display(
                    "Argument {} of function {function} needs to be {}. But {given} that is {given_type} was given",
                    param.name, param.var_type)]
pub struct WrongArgument {
    pub function: Arc<str>,
    pub param: Param,
    pub given: Arc<str>,
    pub given_type: Type,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("mut declared to contain {declared} but initialized with {given} that is {given_type}")]
pub struct WrongInitialization {
    pub declared: Type,
    pub given: Arc<str>,
    pub given_type: Type,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("Condition must be bool but {ins} which is {var_type} was given")]
pub struct WrongCondition {
    pub ins: Arc<str>,
    pub var_type: Type,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display(
    "Cannot access element of {ins} which is {var_type}. Only accessing elements of tuple is possible"
)]
pub struct CannotTupleAccess {
    pub ins: Arc<str>,
    pub var_type: Type,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display(
    "Cannot access field of {ins} which is {var_type}. Only accessing fields of struct is possible"
)]
pub struct CannotFieldAccess {
    pub ins: Arc<str>,
    pub var_type: Type,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display(
    "Cannot slice {ins} which is {var_type}. Only variables of type string | [any] can be sliced"
)]
pub struct CannotSlice {
    pub ins: Arc<str>,
    pub var_type: Type,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("{input} cannot be parsed to variable")]
pub struct CannotBeParsed {
    pub input: Box<str>,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("Cannot index into value of type {var_type}")]
pub struct CannotIndexInto {
    pub var_type: Type,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("Cannot index with {ins}. Index must be int")]
pub struct CannotIndexWith {
    pub ins: Arc<str>,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("{literal} is to big too fit in int type")]
pub struct IntegerOverflow {
    pub literal: Box<str>,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("Cannot create array of length {ins}. Length must be int")]
pub struct WrongLengthType {
    pub ins: Arc<str>,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("Pattern {pattern} is self-contradictory")]
pub struct SelfContradictoryPattern {
    pub pattern: Arc<str>,
}

#[derive(Debug, PartialEq, Eq, Display)]
#[display("Cannot call {ins}. It is not a function")]
pub struct NotAFunction {
    pub ins: Arc<str>,
}
