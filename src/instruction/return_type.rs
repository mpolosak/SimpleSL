use crate as simplesl;
use crate::variable::Type;
use simplesl_macros::var_type;

pub fn return_type_float(lhs: Type, rhs: Type) -> Type {
    if (lhs.matches(&var_type!([int])) && rhs == var_type!(int))
        || (rhs.matches(&var_type!([int])) && lhs == var_type!(int))
    {
        return var_type!([int]);
    }
    if lhs.matches(&var_type!([float])) || rhs.matches(&var_type!([float])) {
        return var_type!([float]);
    }
    if var_type!([int]).matches(&lhs) || var_type!([int]).matches(&rhs) {
        return var_type!([int] | int);
    }
    if var_type!([float]).matches(&lhs) || var_type!([float]).matches(&rhs) {
        return var_type!([float] | float);
    }
    if lhs == var_type!(int) {
        return var_type!(int);
    }
    var_type!(float)
}

pub fn return_type_int(lhs: Type, rhs: Type) -> Type {
    if lhs.matches(&var_type!([any])) || rhs.matches(&var_type!([any])) {
        return var_type!([int]);
    }
    if var_type!([]).matches(&lhs) || var_type!([]).matches(&rhs) {
        return var_type!([int] | int);
    }
    var_type!(int)
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use simplesl_macros::var_type;

    #[test]
    fn return_type_int() {
        use crate::instruction::return_type::return_type_int;
        assert_eq!(
            return_type_int(var_type!(int), var_type!(int)),
            var_type!(int)
        );
        assert_eq!(
            return_type_int(var_type!(float), var_type!(float)),
            var_type!(int)
        );
        assert_eq!(
            return_type_int(var_type!([int]), var_type!(int)),
            var_type!([int])
        );
        assert_eq!(
            return_type_int(var_type!([float]), var_type!(float)),
            var_type!([int])
        );
        assert_eq!(
            return_type_int(var_type!(float), var_type!([float])),
            var_type!([int])
        );
        assert_eq!(
            return_type_int(var_type!(int), var_type!([int] | int)),
            var_type!([int] | int)
        );
        assert_eq!(
            return_type_int(var_type!(float), var_type!([float] | float)),
            var_type!([int] | int)
        );
    }

    #[test]
    fn return_type_float() {
        use crate::instruction::return_type::return_type_float;
        assert_eq!(
            return_type_float(var_type!(int), var_type!(int)),
            var_type!(int)
        );
        assert_eq!(
            return_type_float(var_type!(float), var_type!(float)),
            var_type!(float)
        );
        assert_eq!(
            return_type_float(var_type!([int]), var_type!(int)),
            var_type!([int])
        );
        assert_eq!(
            return_type_float(var_type!(int), var_type!([int])),
            var_type!([int])
        );
        assert_eq!(
            return_type_float(var_type!([float]), var_type!(float)),
            var_type!([float])
        );
        assert_eq!(
            return_type_float(var_type!(float), var_type!([float])),
            var_type!([float])
        );
        assert_eq!(
            return_type_float(var_type!(int), var_type!([int] | int)),
            var_type!([int] | int)
        );
        assert_eq!(
            return_type_float(var_type!([int] | int), var_type!(int)),
            var_type!([int] | int)
        );
        assert_eq!(
            return_type_float(var_type!(float), var_type!([float] | float)),
            var_type!([float] | float)
        );
        assert_eq!(
            return_type_float(var_type!([float] | float), var_type!(float)),
            var_type!([float] | float)
        );
    }
}
