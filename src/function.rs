#[macro_export]
macro_rules! get_vars{
    ($function_name: expr, $params: expr, $($var: ident: $type: ident), *)=>{
        let mut i = 0;
        $(
            let Variable::$type($var) = $params.remove(0) else {
                return Err(format!("{}. argument to function {} should be {}",
                    i, $function_name, stringify!($type)));
            };
            i+=1;
        )*
    }
}

#[macro_export]
macro_rules! add_function{
    ($function_name: expr, $intepreter: ident, $params: ident, $function: expr)=>{
        $intepreter.variables.insert(String::from($function_name), Variable::Function(|$intepreter, $params|{
            $function
        }));
    };
    ($function_name: expr, $intepreter: ident, $params: ident, ($($var: ident: $type: ident), +,) $function: expr)=>{
        $intepreter.variables.insert(String::from($function_name), Variable::Function(|$intepreter, mut $params|{
            if $params.len()<count!($($var),*){
                return Err(format!("Function {} requiers at least {} arguments", $function_name, count!($($var),*)));
            }
            get_vars!($function_name, $params, $($var: $type), +);
            $function
        }));
    };
    ($function_name: expr, $intepreter: ident, $params: ident, only ($($var: ident: $type: ident), +,) $function: expr)=>{
        $intepreter.variables.insert(String::from($function_name), Variable::Function(|$intepreter, mut $params|{
            if $params.len()!=count!($($var),*){
                return Err(format!("Function {} requiers {} arguments", $function_name, count!($($var),*)));
            }
            get_vars!($function_name, $params, $($var: $type), +);
            $function
        }));
    }
}

#[macro_export]
macro_rules! count {
    ($h:expr) => (1);
    ($h:expr, $($t:expr),*) =>
        (1 + count!($($t),*));
}