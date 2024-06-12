use super::{
    bin_op::*,
    prefix_op::{BitwiseNot, Not, UnaryMinus},
};
use crate as simplesl;
use crate::variable::{ReturnType, Type};
use duplicate::duplicate_item;
use simplesl_macros::var_type;

#[duplicate_item(T; [Not]; [BitwiseNot]; [UnaryMinus])]
impl ReturnType for T {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}

#[duplicate_item(T; [Multiply]; [Divide]; [Pow]; [Subtract])]
impl ReturnType for T {
    fn return_type(&self) -> Type {
        let lhs = self.lhs.return_type();
        let rhs = self.rhs.return_type();
        return_type_float(lhs, rhs)
    }
}

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

#[duplicate_item(
    T;
    [Greater]; [GreaterOrEqual]; [Lower]; [LowerOrEqual];
    [BitwiseAnd]; [BitwiseOr]; [And]; [Or]; [Xor];
    [Modulo]; [LShift]; [RShift]
)]
impl ReturnType for T {
    fn return_type(&self) -> Type {
        let lhs = self.lhs.return_type();
        let rhs = self.rhs.return_type();
        return_type_int(lhs, rhs)
    }
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

#[duplicate_item(T; [Equal]; [NotEqual])]
impl ReturnType for T {
    fn return_type(&self) -> Type {
        var_type!(int)
    }
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
