#[macro_export]
macro_rules! params {
    ($($name: literal: $type_name: expr),*)=>{
        vec!(
            $(
                Param{name: String::from($name), var_type: $type_name},
            )*
        )
    }
}
