use crate::function::{NativeFunction, Param};
use crate::intepreter::VariableMap;
use crate::variable::Variable;

pub fn add_array_functions(variables: &mut VariableMap){
    variables.add_native_function("array_at", NativeFunction{
        params: vec!(
            Param::new("array", "Array"),
            Param::new("index", "Float"),
        ),
        body: |name, _intepreter, args|{
            let Variable::Array(array) = args.get("array")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            let Variable::Float(findex) = args.get("index")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            if findex.fract()!=0.0{
                return Err(String::from("Index must be integer"))
            }
            if findex<0.0 {
                return Err(String::from("Index must be higher than 0"))
            }
            let index = findex as usize;
            if index>=array.len(){
                Err(String::from("Index must be lower than array size"))
            } else {
                Ok(array[index].clone())
            }
        }
    });
}