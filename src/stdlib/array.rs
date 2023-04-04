use crate::error::Error;
use crate::function::{NativeFunction, Param};
use crate::intepreter::VariableMap;
use crate::variable::Variable;
use crate::params;

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
}