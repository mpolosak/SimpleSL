use simplesl::{variable::Typed, Code, Error, Interpreter};
use simplesl_macros::{var, var_type};
use std::fs;

#[test]
fn test_example1() -> Result<(), Error> {
    let interpreter = Interpreter::with_stdlib();
    let example1 = fs::read_to_string("example_scripts/example1")?;
    let result = Code::parse(&interpreter, &example1)?.exec()?;
    assert_eq!(result, var!(()));
    Ok(())
}

#[test]
fn test_fib() -> Result<(), Error> {
    let mut interpreter = Interpreter::with_stdlib();
    let fib = fs::read_to_string("example_scripts/fib")?;
    let result = Code::parse(&interpreter, &fib)?.exec_unscoped(&mut interpreter)?;
    assert_eq!(result.as_type(), var_type!((int)->[int]));
    let array_fib = result.into_function().unwrap();
    assert_eq!(
        array_fib.create_call([var!(3)].into())?.exec()?,
        var!([0, 1, 1])
    );

    let int = interpreter.get_variable("int").unwrap();
    assert_eq!(int.as_type(), var_type!((any, int)->int));
    let int = int.clone().into_function().unwrap();
    assert_eq!(
        int.clone().create_call([var!(3), var!(0)].into())?.exec()?,
        var!(3)
    );
    assert_eq!(
        int.create_call([var!(3.0), var!(0)].into())?.exec()?,
        var!(0)
    );

    let custom_fib = interpreter.get_variable("custom_fib").unwrap();
    assert_eq!(custom_fib.as_type(), var_type!((int, int, int)->int));

    let fib = interpreter.get_variable("fib").unwrap();
    assert_eq!(fib.as_type(), var_type!((int)->int));
    let fib = fib.clone().into_function().unwrap();
    assert_eq!(fib.create_call([var!(8)].into())?.exec()?, var!(21));

    let custom_array_fib = interpreter.get_variable("custom_array_fib").unwrap();
    assert_eq!(custom_array_fib.as_type(), var_type!(([int], int)->[int]));

    let _ = interpreter.get_variable("array_fib").unwrap();
    Ok(())
}

#[test]
fn test_fizzbuzz() -> Result<(), Error> {
    let mut interpreter = Interpreter::with_stdlib();
    let fizzbuzz = fs::read_to_string("example_scripts/fizzbuzz")?;
    let result = Code::parse(&interpreter, &fizzbuzz)?.exec_unscoped(&mut interpreter)?;
    assert_eq!(result.as_type(), var_type!([()]));
    let fizzbuzz = interpreter.get_variable("fizzbuzz").unwrap();
    assert_eq!(fizzbuzz.as_type(), var_type!((int)->(int|string)));
    let fizzbuzz = fizzbuzz.clone().into_function().unwrap();
    assert_eq!(
        fizzbuzz.clone().create_call([var!(3)].into())?.exec()?,
        var!("Fizz")
    );

    let iota = interpreter.get_variable("iota").unwrap();
    assert_eq!(iota.as_type(), var_type!((int,int)->[int]));
    let iota = iota.clone().into_function().unwrap();
    assert_eq!(
        iota.create_call([var!(3), var!(3)].into())?.exec()?,
        var!([3, 4, 5])
    );
    Ok(())
}

#[test]
fn test_quick_sort() -> Result<(), Error> {
    let mut interpreter = Interpreter::with_stdlib();
    let quick_sort = fs::read_to_string("example_scripts/quick_sort")?;
    let result = Code::parse(&interpreter, &quick_sort)?.exec_unscoped(&mut interpreter)?;
    assert_eq!(result.as_type(), var_type!(([int])->[int]));
    let sort = interpreter
        .get_variable("sort")
        .unwrap()
        .clone()
        .into_function()
        .unwrap();
    assert_eq!(
        sort.create_call([var!([9, 3, 6])].into())?.exec()?,
        var!([3, 6, 9])
    );
    Ok(())
}

#[test]
fn test_replace() -> Result<(), Error> {
    let mut interpreter = Interpreter::with_stdlib();
    let replace = fs::read_to_string("example_scripts/replace")?;
    let result = Code::parse(&interpreter, &replace)?.exec_unscoped(&mut interpreter)?;
    assert_eq!(result.as_type(), var_type!(([any], int, int)->[any]));
    let replace = interpreter
        .get_variable("replace")
        .unwrap()
        .clone()
        .into_function()
        .unwrap();
    assert_eq!(
        replace
            .create_call([var!([9, 3, 6]), var!(0), var!(2)].into())?
            .exec()?,
        var!([6, 3, 9])
    );
    Ok(())
}
