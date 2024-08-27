use crate as simplesl;
use crate::variable::Type;
use simplesl_macros::var_type;

pub fn return_type_bool(lhs: Type, rhs: Type) -> Type {
    if lhs.matches(&var_type!([any])) || rhs.matches(&var_type!([any])) {
        return var_type!([bool]);
    }
    if var_type!([]).matches(&lhs) || var_type!([]).matches(&rhs) {
        return var_type!([bool] | bool);
    }
    var_type!(bool)
}

#[cfg(test)]
mod tests {
    use crate::{self as simplesl};
    use simplesl_macros::var_type;

    #[test]
    fn return_type_bool() {
        use crate::instruction::return_type::return_type_bool;
        assert_eq!(
            return_type_bool(var_type!(int), var_type!(int)),
            var_type!(bool)
        );
        assert_eq!(
            return_type_bool(var_type!(float), var_type!(float)),
            var_type!(bool)
        );
        assert_eq!(
            return_type_bool(var_type!([int]), var_type!(int)),
            var_type!([bool])
        );
        assert_eq!(
            return_type_bool(var_type!([float]), var_type!(float)),
            var_type!([bool])
        );
        assert_eq!(
            return_type_bool(var_type!(float), var_type!([float])),
            var_type!([bool])
        );
        assert_eq!(
            return_type_bool(var_type!(int), var_type!([int] | int)),
            var_type!([bool] | bool)
        );
        assert_eq!(
            return_type_bool(var_type!(float), var_type!([float] | float)),
            var_type!([bool] | bool)
        );
    }
}
