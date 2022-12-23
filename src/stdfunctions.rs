use crate::variable::*;
use crate::*;

pub fn add_std_functions(variables: &mut VariableMap){
    add_function!("add", variables, params, (var1: Float, var2: Float,) {
        Ok(Variable::Float(var1+var2))
    });
    add_function!("subtract", variables, params, (var1: Float, var2: Float,){
        Ok(Variable::Float(var1-var2))
    });
    add_function!("multiply", variables, params, (var1: Float, var2: Float,){
        Ok(Variable::Float(var1*var2))
    });
    add_function!("divide", variables, params, (var1: Float, var2: Float,){
        Ok(Variable::Float(var1/var2))
    });
    add_function!("modulo", variables, params, (var1: Float, var2: Float,){
        let divided = var1/var2;
        let result = var1 - var2*divided.floor();
        Ok(Variable::Float(result))
    });
    add_function!("or", variables, params, (var1: Float, var2: Float,){
        Ok(Variable::Float(var1.abs()+var2.abs()))
    });
    add_function!("not", variables, params, (var: Float,){
        Ok(Variable::Float(if var==0.0{1.0}else{0.0}))
    });
    add_function!("if", variables, params, (condition: Float, function: Function,){
        if condition == 0.0 { return Ok(Variable::Null)};
        return function(variables, params);
    });
    // add_function!("while", variables, params, {
    //     loop{
    //         let condition = get_var!("while", variables, params, 0, Float);
    //         if condition == 0.0 { break };
    //         let function = get_var!("while", variables, params, 1, Function);
    //         let function_params = if let Some(fparams) = params.get(2..) { fparams.to_vec() }
    //                               else { Array::new() };
    //         if let Err(error) = function(variables, function_params){
    //             return Err(error)
    //         }
    //     }
    //     return Ok(Variable::Null)
    // });
}