#![cfg(test)]
use std::fs;

use crate::{
    variable::{FunctionType, Type, Typed, Variable},
    Code, Interpreter, Result,
};
#[test]
fn test_example1() -> Result<()> {
    let interpreter = Interpreter::with_stdlib();
    let example1 = fs::read_to_string("example_scripts/example1")?;
    let result = Code::parse(&interpreter, &example1)?.exec()?;
    assert_eq!(result, Variable::Void);
    Ok(())
}

#[test]
fn test_fib() -> Result<()> {
    let mut interpreter = Interpreter::with_stdlib();
    let fib = fs::read_to_string("example_scripts/fib")?;
    let result = Code::parse(&interpreter, &fib)?.exec_unscoped(&mut interpreter)?;
    assert_eq!(
        result.as_type(),
        FunctionType {
            params: [Type::Int].into(),
            return_type: [Type::Int].into()
        }
        .into()
    );
    let Variable::Function(array_fib) = result else {
        panic!()
    };
    assert_eq!(
        array_fib.create_call([Variable::Int(3)].into())?.exec()?,
        [Variable::Int(0), Variable::Int(1), Variable::Int(1)].into()
    );

    let int = interpreter.get_variable("int").unwrap();
    assert_eq!(
        int.as_type(),
        FunctionType {
            params: [Type::Any, Type::Int].into(),
            return_type: Type::Int
        }
        .into()
    );
    let Variable::Function(int) = int.clone() else {
        panic!()
    };
    assert_eq!(
        int.clone()
            .create_call([Variable::Int(3), Variable::Int(0)].into())?
            .exec()?,
        Variable::Int(3)
    );
    assert_eq!(
        int.create_call([Variable::Float(3.0), Variable::Int(0)].into())?
            .exec()?,
        Variable::Int(0)
    );

    let custom_fib = interpreter.get_variable("custom_fib").unwrap();
    assert_eq!(
        custom_fib.as_type(),
        FunctionType {
            params: [Type::Int, Type::Int, Type::Int].into(),
            return_type: Type::Int
        }
        .into()
    );

    let fib = interpreter.get_variable("fib").unwrap();
    assert_eq!(
        fib.as_type(),
        FunctionType {
            params: [Type::Int].into(),
            return_type: Type::Int
        }
        .into()
    );
    let Variable::Function(fib) = fib.clone() else {
        panic!()
    };
    assert_eq!(
        fib.create_call([Variable::Int(8)].into())?.exec()?,
        Variable::Int(21)
    );

    let custom_array_fib = interpreter.get_variable("custom_array_fib").unwrap();
    assert_eq!(
        custom_array_fib.as_type(),
        FunctionType {
            params: [[Type::Int].into(), Type::Int].into(),
            return_type: [Type::Int].into()
        }
        .into()
    );

    let _ = interpreter.get_variable("array_fib").unwrap();
    Ok(())
}

#[test]
fn test_fizzbuzz() -> Result<()> {
    let mut interpreter = Interpreter::with_stdlib();
    let fizzbuzz = fs::read_to_string("example_scripts/fizzbuzz")?;
    let result = Code::parse(&interpreter, &fizzbuzz)?.exec_unscoped(&mut interpreter)?;
    assert_eq!(result.as_type(), [Type::Void].into());

    let fizzbuzz = interpreter.get_variable("fizzbuzz").unwrap();
    assert_eq!(
        fizzbuzz.as_type(),
        FunctionType {
            params: [Type::Int].into(),
            return_type: Type::Int | Type::String
        }
        .into()
    );
    let Variable::Function(int) = fizzbuzz.clone() else {
        panic!()
    };
    assert_eq!(
        int.clone().create_call([Variable::Int(3)].into())?.exec()?,
        Variable::String("Fizz".into())
    );

    let iota = interpreter.get_variable("iota").unwrap();
    assert_eq!(
        iota.as_type(),
        FunctionType {
            params: [Type::Int, Type::Int].into(),
            return_type: [Type::Int].into()
        }
        .into()
    );
    let Variable::Function(iota) = iota.clone() else {
        panic!()
    };
    assert_eq!(
        iota.create_call([Variable::Int(3), Variable::Int(3)].into())?
            .exec()?,
        [Variable::Int(3), Variable::Int(4), Variable::Int(5)].into()
    );
    Ok(())
}
