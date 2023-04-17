use crate::function::{NativeFunction, Param};
use crate::{intepreter::VariableMap, variable::{Variable,Array},
    error::Error, params};

pub fn add_array_functions(variables: &mut VariableMap){
    variables.add_native_function("array_at", NativeFunction{
        params: params!("array":"array", "index":"float"),
        body: |_name, _intepreter, args|{
            let Variable::Array(array) = args.get("array")? else {
                panic!();
            };
            let Variable::Float(findex) = args.get("index")? else {
                panic!();
            };
            if findex.fract()!=0.0 || findex<0.0 {
                return Err(Error::WrongType(
                    String::from("index"),
                    String::from("natural")
                ));
            }
            let index = findex as usize;
            if index>=array.len(){
                Err(Error::IndexToBig)
            } else {
                Ok(array[index].clone())
            }
        }
    });
    variables.add_native_function("array_concat", NativeFunction {
        params: params!("array1":"array","array2":"array"),
        body: |_name,_intepreter,args|{
            let Variable::Array(array1) = args.get("array1")? else {
                panic!()
            };
            let Variable::Array(array2) = args.get("array2")? else {
                panic!()
            };
            let mut new_array = (*array1).clone();
            for element in array2.iter() {
                new_array.push(element.clone());
            }
            Ok(Variable::Array(new_array.into()))
        }
    });
    variables.add_native_function("array_len", NativeFunction {
        params: params!("array":"array"),
        body: |_name, _intepreter, args|{
            let Variable::Array(array) = args.get("array")? else {
                panic!()
            };
            Ok(Variable::Float(array.len() as f64))
        }
    });
    variables.add_native_function("for_each", NativeFunction {
        params: params!("array":"array", "function":"function"),
        body: |_name, intepreter, args|{
            let Variable::Array(array) = args.get("array")? else {
                panic!()
            };
            let Variable::Function(function) = args.get("function")? else {
                panic!()
            };
            let mut new_array = Array::new();
            for var in array.iter() {
                new_array.push(function.exec(String::from("function"),
                    intepreter, vec![var.clone()])?);
            }
            Ok(Variable::Array(new_array.into()))
        }
    });
}