use crate::{
    function::{NativeFunction, Param, Params},
    intepreter::VariableMap,
    params,
    variable_type::{GetType, Type},
};

pub fn add_types_functions(variables: &mut VariableMap) {
    variables.add_native_function(
        "float",
        NativeFunction {
            params: Params {
                standard: params!("value": Type::Any, "fallback": Type::Float),
                catch_rest: None,
            },
            return_type: Type::Float,
            body: |_name, _intepreter, args| {
                let value = args.get("value")?;
                let fallback = args.get("fallback")?;
                if value.get_type() == Type::Float {
                    Ok(value)
                } else {
                    Ok(fallback)
                }
            },
        },
    );

    variables.add_native_function(
        "string",
        NativeFunction {
            params: Params {
                standard: params!("value": Type::Any, "fallback": Type::String),
                catch_rest: None,
            },
            return_type: Type::String,
            body: |_name, _intepreter, args| {
                let value = args.get("value")?;
                let fallback = args.get("fallback")?;
                if value.get_type() == Type::String {
                    Ok(value)
                } else {
                    Ok(fallback)
                }
            },
        },
    );

    variables.add_native_function(
        "array",
        NativeFunction {
            params: Params {
                standard: params!("value": Type::Any, "fallback": Type::Array),
                catch_rest: None,
            },
            return_type: Type::Array,
            body: |_name, _intepreter, args| {
                let value = args.get("value")?;
                let fallback = args.get("fallback")?;
                if value.get_type() == Type::Array {
                    Ok(value)
                } else {
                    Ok(fallback)
                }
            },
        },
    );
}
