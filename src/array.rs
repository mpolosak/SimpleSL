use crate::function::{NativeFunction, Param};
use crate::intepreter::Intepreter;
use crate::variable::{Variable, Array};

pub fn add_array_functions(intepreter: &mut Intepreter){
    intepreter.add_function("array_at", NativeFunction{
        params: vec!(
            Param::new("array", "Array"),
            Param::new("index", "Float"),
        ),
        body: |_name, _intepreter, args|{
            let Some(Variable::Array(array)) = args.get("array") else {
                return Err(String::from("Something strange happend"))
            };
            let Some(Variable::Float(findex)) = args.get("index") else {
                return Err(String::from("Something strange happend"))
            };
            if findex.fract()!=0.0{
                return Err(String::from("Index must be integer"))
            }
            if *findex<0.0 {
                return Err(String::from("Index must be higher than 0"))
            }
            let index = *findex as usize;
            if index>=array.len(){
                Err(String::from("Index must be lower than array size"))
            } else {
                Ok(array[index].clone())
            }
        }
    });
}