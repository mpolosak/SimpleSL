#[macro_export]
macro_rules! get_vars{
    ($function_name: expr, $variables: expr, $params: expr, $($var: ident: $type: ident), *)=>{
        if $params.len()!=count!($($var),*){
            return Err(format!("Function {} requiers 2 arguments", $function_name));
        }
        let mut i = 0;
        $(
            let $var = match &$params[i]{
                Param::$type(value) => *value,
                Param::Variable(name) => match $variables.get(name) {
                    Some(Variable::$type(value)) => *value,
                    Some(_) => return Err(format!("Function {} requiers float", $function_name)),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(format!("Function {} requiers float", $function_name))
            };
            i+=1;
        )*
    }
}

#[macro_export]
macro_rules! count {
    ($h:expr) => (1);
    ($h:expr, $($t:expr),*) =>
        (1 + count!($($t),*));
}