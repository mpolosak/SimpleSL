use crate::{
    self as simplesl, function::Function, interpreter::VariableMap, stdlib::len, variable::{Array, Typed, Variable}, ExecError
};
use simplesl_macros::{decls, export, var};
use std::sync::Arc;

#[export(Slice)]
#[return_type([any]|string)]
fn slice(
    #[var_type([any]|string)] variable: Variable,
    start: Option<isize>,
    end: Option<isize>,
    step: Option<isize>,
) -> Variable {
    let s = slyce::Slice {
        start: start.into(),
        end: end.into(),
        step,
    };
    if let Variable::String(variable) = variable {
        let chars: Box<[char]> = variable.chars().collect();
        let result: String = s.apply(&chars).cloned().collect();
        return result.into();
    }
    let array = variable.into_array().unwrap();
    let result: Arc<[Variable]> = s.apply(array.as_ref()).cloned().collect();
    result.into()
}

#[export(ArrayRepeat)]
#[return_type([any])]
fn array_repeat(value: Variable, len: i64) -> Result<Variable, ExecError> {
    if len < 0 {
        return Err(ExecError::NegativeLength);
    }

    Ok(var!([value; len]))
}

#[export(At)]
fn at(#[var_type([any]|string)] variable: &Variable, index: i64) -> Result<Variable, ExecError> {
    let index = if index >= 0 {
        index as usize
    } else {
        let index = len(variable) as i64 + index;
        if index < 0 {
            return Err(ExecError::IndexOutOfBounds);
        }
        index as usize
    };
    match variable {
        Variable::String(string) => string
            .chars()
            .nth(index)
            .ok_or(ExecError::IndexOutOfBounds)
            .map(|ch| ch.to_string().into()),
        Variable::Array(array) => array.get(index).ok_or(ExecError::IndexOutOfBounds).cloned(),
        Variable::Tuple(tuple) => tuple.get(index).ok_or(ExecError::IndexOutOfBounds).cloned(),
        variable => unreachable!("Tried to index into {}", variable.as_type()),
    }
}

#[export(GetField)]
fn get_field(variable: &VariableMap, field: &str) -> Option<Variable> {
    variable.get(field).cloned()
}

#[export(Collect)]
fn collect(#[var_type(() -> (bool, any))] iter: &Arc<Function>) -> Result<Array, ExecError> {
    let mut vec = Vec::new();
    while let Variable::Tuple(tuple) = iter.exec_with_args(&[])? && tuple[0] == Variable::Bool(true){
        vec.push(tuple[1].clone());
    }
    Ok(vec.into())
}

#[export(Deref)]
fn deref(variable: Variable) -> Variable {
    if let Variable::Mut(var) = variable {
        return var.variable.read().unwrap().clone()
    }
    variable
}

decls! {
    AND:=(iter: () -> (bool, int)) -> int {
        return iter $!0 (acc: int, curr: int) -> int {
            return acc & curr;
        }
    }
    OR:=(iter: () -> (bool, int)) -> int {
        return iter $0 (acc: int, curr: int) -> int {
            return acc | curr;
        }
    }
    ALL:=(iter: () -> (bool, bool)) -> bool {
        for value in iter {
            if !value { return false; }
        }
        return true;
    }
    ANY:=(iter: () -> (bool, bool)) -> bool {
        for value in iter {
            if value { return true; }
        }
        return false;
    }
    INT_PRODUCT:=(iter: () -> (bool, int)) -> int {
        return iter $1 (acc: int, curr: int) -> int {
            return acc * curr;
        }
    }
    FLOAT_PRODUCT:=(iter: () -> (bool, float)) -> float {
        return iter $1.0 (acc: float, curr: float) -> float {
            return acc * curr;
        }
    }
    INT_SUM:=(iter: () -> (bool, int)) -> int {
        return iter $0 (acc: int, curr: int) -> int {
            return acc + curr;
        }
    }
    FLOAT_SUM:=(iter: () -> (bool, float)) -> float {
        return iter $0.0 (acc: float, curr: float) -> float {
            return acc + curr;
        }
    }
    STRING_SUM:=(iter: () -> (bool, string)) -> string {
        return iter $"" (acc: string, curr: string) -> string {
            return acc + curr;
        }
    }
    REDUCE:=(iter: () -> (bool, any), initial_value: any, function: (any, any) -> any) -> any {
        result:=mut initial_value;
        for element in iter {
            result=function(*result, element)
        }
        return *result
    }
    FILTER:=(iter: () -> (bool, any), predicate: (any) -> bool) -> () -> (bool, any) {
        return () -> (bool, any) {
            loop {
                res := iter();
                (con, value) := res;
                if !con || predicate(value) return res;
            }
            return (false, 0);
        }
    }
    Operators:=struct{
        bitand_reduce := AND,
        bitor_reduce := OR,
        all := ALL,
        any := ANY,
        int_product := INT_PRODUCT,
        float_product := FLOAT_PRODUCT,
        int_sum := INT_SUM,
        float_sum := FLOAT_SUM,
        string_sum := STRING_SUM,
        reduce := REDUCE,
        collect := Collect,
        filter := FILTER,
        slice := Slice,
        array_repeat := ArrayRepeat,
        at := At,
        get_field := GetField,
        deref := Deref
    }
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::ExecError;
    use crate::stdlib::operators::at;
    use simplesl_macros::var;

    #[test]
    fn check_at() {
        let array = var!([4, 5.5, "var"]);
        assert_eq!(at(&array, 0), Ok(var!(4)));
        assert_eq!(at(&array, 1), Ok(var!(5.5)));
        assert_eq!(at(&array, 2), Ok(var!("var")));
        assert_eq!(at(&array, -1), Ok(var!("var")));
        assert_eq!(at(&array, 3), Err(ExecError::IndexOutOfBounds));
        let string = var!("tex");
        assert_eq!(at(&string, 0), Ok(var!("t")));
        assert_eq!(at(&string, 2), Ok(var!("x")));
        assert_eq!(at(&string, 3), Err(ExecError::IndexOutOfBounds));
        assert_eq!(at(&string, -1), Ok(var!("x")))
    }
}
