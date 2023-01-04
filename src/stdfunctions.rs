use crate::variable::*;
use crate::*;

pub fn add_std_functions(intepreter: &mut Intepreter){
    add_function!("import", intepreter, args, only (path: Text,) {
        intepreter.load_and_exec(path)
    });
    add_function!("add", intepreter, args, only (var1: Float, var2: Float,) {
        Ok(Variable::Float(var1+var2))
    });
    add_function!("subtract", intepreter, args, only (var1: Float, var2: Float,){
        Ok(Variable::Float(var1-var2))
    });
    add_function!("multiply", intepreter, args, only (var1: Float, var2: Float,){
        Ok(Variable::Float(var1*var2))
    });
    add_function!("divide", intepreter, args, only (var1: Float, var2: Float,){
        Ok(Variable::Float(var1/var2))
    });
    add_function!("modulo", intepreter, args, only (var1: Float, var2: Float,){
        let divided = var1/var2;
        let result = var1 - var2*divided.floor();
        Ok(Variable::Float(result))
    });
    add_function!("or", intepreter, args, only (var1: Float, var2: Float,){
        Ok(Variable::Float(var1.abs()+var2.abs()))
    });
    add_function!("not", intepreter, args, only (var: Float,){
        Ok(Variable::Float(if var==0.0{1.0}else{0.0}))
    });
    add_function!("if", intepreter, args, (condition: Float, function: Function,){
        if condition == 0.0 { return Ok(Variable::Null)};
        return function(intepreter, args);
    });
    // add_function!("while", intepreter, args, {
    //     loop{
    //         let condition = get_var!("while", intepreter, args, 0, Float);
    //         if condition == 0.0 { break };
    //         let function = get_var!("while", intepreter, args, 1, Function);
    //         let function_args = if let Some(fargs) = args.get(2..) { fargs.to_vec() }
    //                               else { Array::new() };
    //         if let Err(error) = function(intepreter, function_args){
    //             return Err(error)
    //         }
    //     }
    //     return Ok(Variable::Null)
    // });
}