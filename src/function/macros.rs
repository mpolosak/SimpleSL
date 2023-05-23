#[macro_export]
macro_rules! params {
    ($($name: literal: $type_name: expr),*)=>{
        vec!(
            $(
                Param::Standard(String::from($name), $type_name),
            )*
        )
    }
}
