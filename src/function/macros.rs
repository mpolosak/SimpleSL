#[macro_export]
macro_rules! params {
    ($($name: literal: $type_name: literal),*)=>{
        vec!(
            $(
                Param::new($name, $type_name),
            )*
        )
    }
}