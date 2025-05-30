use crate::instruction::{Instruction, create_from_instructions_with_exec};
use crate::variable::{Array, Type, Variable};
use crate::{self as simplesl, BinOperator};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    static ref ACCEPTED_TYPE: Type =
        var_type!((int, int) | (float, float) | (string, string) | ([any], [any]));
}

pub(crate) fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
    let lhs = lhs.clone();
    let rhs = rhs.clone();
    var_type!((lhs, rhs)).matches(&ACCEPTED_TYPE)
}

pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
    create_from_instructions_with_exec(lhs, rhs, BinOperator::Add, exec)
}

pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
    match (lhs, rhs) {
        (Variable::Int(value1), Variable::Int(value2)) => value1.wrapping_add(value2).into(),
        (Variable::Float(value1), Variable::Float(value2)) => (value1 + value2).into(),
        (Variable::String(value1), Variable::String(value2)) => format!("{value1}{value2}").into(),
        (Variable::Array(array1), Variable::Array(array2)) => Array::concat(array1, array2).into(),
        (lhs, rhs) => panic!("Tried to do {lhs} + {rhs} which is imposible"),
    }
}

pub fn return_type(lhs: Type, rhs: Type) -> Type {
    let Some(lhs_element) = lhs.element_type() else {
        return lhs;
    };
    let rhs_element = rhs.element_type().unwrap();
    var_type!([lhs_element | rhs_element])
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, BinOperator, Code, Error, Interpreter, instruction::bin_op::add,
        variable::Variable,
    };
    use simplesl_macros::{var, var_type};

    #[test]
    fn test_add_operator() {
        assert_eq!(parse_and_exec("4+4"), Ok(Variable::Int(8)));
        assert_eq!(parse_and_exec("4.5+0.5"), Ok(Variable::Float(5.0)));
        assert_eq!(parse_and_exec(r#""aa" + "B""#), Ok(var!("aaB")));
        assert_eq!(parse_and_exec("[5, 5, 6] + [5]"), Ok(var!([5, 5, 6, 5])));
        assert_eq!(
            parse_and_exec(r#"[4, 5, 6] + 5"#),
            Err(Error::CannotDo2(
                var_type!([int]),
                BinOperator::Add,
                var_type!(int)
            ))
        );
        assert_eq!(
            parse_and_exec(r#"[4.5, 5.7, 6.0] + 3.3"#),
            Err(Error::CannotDo2(
                var_type!([float]),
                BinOperator::Add,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#""7" + ["a", "aaa"]"#),
            Err(Error::CannotDo2(
                var_type!(string),
                BinOperator::Add,
                var_type!([string])
            ))
        );
        assert_eq!(
            parse_and_exec(r#"["a", "aaa"]+"3""#),
            Err(Error::CannotDo2(
                var_type!([string]),
                BinOperator::Add,
                var_type!(string)
            ))
        );
        assert_eq!(
            parse_and_exec("4+4.5"),
            Err(Error::CannotDo2(
                var_type!(int),
                BinOperator::Add,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#""4"+4.5"#),
            Err(Error::CannotDo2(
                var_type!(string),
                BinOperator::Add,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#""4"+4"#),
            Err(Error::CannotDo2(
                var_type!(string),
                BinOperator::Add,
                var_type!(int)
            ))
        );
        assert_eq!(
            parse_and_exec(r#"[4]+4.5"#),
            Err(Error::CannotDo2(
                var_type!([int]),
                BinOperator::Add,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#"[4, 5.5]+4.5"#),
            Err(Error::CannotDo2(
                var_type!([int | float]),
                BinOperator::Add,
                var_type!(float)
            ))
        )
    }

    #[test]
    fn return_type() {
        // int
        assert_eq!(
            add::return_type(var_type!(int), var_type!(int)),
            var_type!(int)
        );
        // float
        assert_eq!(
            add::return_type(var_type!(float), var_type!(float)),
            var_type!(float)
        );
        // string
        assert_eq!(
            add::return_type(var_type!(string), var_type!(string)),
            var_type!(string)
        );
        // array + array
        assert_eq!(
            add::return_type(var_type!([int]), var_type!([int])),
            var_type!([int])
        );
        assert_eq!(
            add::return_type(var_type!([float]), var_type!([float])),
            var_type!([float])
        );
        assert_eq!(
            add::return_type(var_type!([string]), var_type!([string])),
            var_type!([string])
        );
        assert_eq!(
            add::return_type(var_type!([int]), var_type!([float])),
            var_type!([float | int])
        );
        assert_eq!(
            add::return_type(var_type!([int]), var_type!([any])),
            var_type!([any])
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, crate::Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
