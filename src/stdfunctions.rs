use crate::variable::*;
use crate::*;

pub fn add_std_functions(intepreter: &mut Intepreter){
    add_function!("import", intepreter, params, only (path: Text,) {
        intepreter.load_and_exec(path)
    });
    add_function!("add", intepreter, params, only (var1: Float, var2: Float,) {
        Ok(Variable::Float(var1+var2))
    });
    add_function!("subtract", intepreter, params, only (var1: Float, var2: Float,){
        Ok(Variable::Float(var1-var2))
    });
    add_function!("multiply", intepreter, params, only (var1: Float, var2: Float,){
        Ok(Variable::Float(var1*var2))
    });
    add_function!("divide", intepreter, params, only (var1: Float, var2: Float,){
        Ok(Variable::Float(var1/var2))
    });
    add_function!("modulo", intepreter, params, only (var1: Float, var2: Float,){
        let divided = var1/var2;
        let result = var1 - var2*divided.floor();
        Ok(Variable::Float(result))
    });
    add_function!("or", intepreter, params, only (var1: Float, var2: Float,){
        Ok(Variable::Float(var1.abs()+var2.abs()))
    });
    add_function!("not", intepreter, params, only (var: Float,){
        Ok(Variable::Float(if var==0.0{1.0}else{0.0}))
    });
    add_function!("if", intepreter, params, (condition: Float, function: Function,){
        if condition == 0.0 { return Ok(Variable::Null)};
        return function(intepreter, params);
    });
    // add_function!("while", intepreter, params, {
    //     loop{
    //         let condition = get_var!("while", intepreter, params, 0, Float);
    //         if condition == 0.0 { break };
    //         let function = get_var!("while", intepreter, params, 1, Function);
    //         let function_params = if let Some(fparams) = params.get(2..) { fparams.to_vec() }
    //                               else { Array::new() };
    //         if let Err(error) = function(intepreter, function_params){
    //             return Err(error)
    //         }
    //     }
    //     return Ok(Variable::Null)
    // });
}