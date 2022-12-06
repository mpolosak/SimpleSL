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
    ($variables: expr, $param: expr)=>{
        {
            match &$param{
                Param::Float(value) => Variable::Float(*value),
                Param::Text(value) => Variable::Text(value.clone()),
                Param::Variable(name) => match $variables.get(name) {
                    Some(variable) => variable.clone(),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
            }
        }
    };
}

#[macro_export]
macro_rules! add_function{
    ($function_name: expr, $variables: ident, $params: ident, $function: expr)=>{
        $variables.insert(String::from($function_name), Variable::Function(|$variables, $params|{
            $function
        }));
    };
    ($function_name: expr, $variables: ident, $params: ident, ($($var: ident: $type: ident), +,) $function: expr)=>{
        $variables.insert(String::from($function_name), Variable::Function(|$variables, $params|{
            get_vars!($function_name, $variables, $params, $($var: $type), +);
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