#[macro_export]
macro_rules! get_vars{
    ($function_name: expr, $variables: expr, $params: expr, $($var: ident: $type: ident), *)=>{
        if $params.len()!=count!($($var),*){
            return Err(format!("Function {} requiers 2 arguments", $function_name));
        }
        let mut i = 0;
        $(
            let $var = get_var!($function_name, $variables, $params, i, $type);
            i+=1;
        )*
    }
}

#[macro_export]
macro_rules! get_var{
    ($function_name: expr, $variables: expr, $params: expr, $i: expr, Function)=>{
        {
            match &$params[$i]{
                Param::Variable(name) => match $variables.get(name) {
                    Some(Variable::Function(func)) => *func,
                    Some(_) => return Err(format!("{} argument to {} should be function", $i+1, $function_name)),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(format!("{} argument to {} should be function", $i+1, $function_name))
            }
        }
    };
    ($function_name: expr, $variables: expr, $params: expr, $i: expr, $type: ident)=>{
        {
            match &$params[$i]{
                Param::$type(value) => *value,
                Param::Variable(name) => match $variables.get(name) {
                    Some(Variable::$type(value)) => *value,
                    Some(_) => return Err(format!("{} argument to {} should be {}", $i+1, $function_name, stringify!($type))),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(format!("{} argument to {} should be {}", $i+1, $function_name, stringify!($type)))
            }
        }
    };
}

#[macro_export]
macro_rules! count {
    ($h:expr) => (1);
    ($h:expr, $($t:expr),*) =>
        (1 + count!($($t),*));
}