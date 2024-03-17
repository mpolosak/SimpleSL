use super::{
    macros::{binOpCBU, bin_num_op::ACCEPTED_TYPE},
    traits::Exec,
    Instruction,
};
use crate::variable::{ReturnType, Type, Variable};
use crate::Interpreter;
use duplicate::duplicate_item;
use match_any::match_any;

binOpCBU!(Greater, ">");
binOpCBU!(GreaterOrEqual, ">=");
binOpCBU!(Lower, "<");
binOpCBU!(LowerOrEqual, "<=");

#[duplicate_item(
    ord op;
    [Greater] [>]; [GreaterOrEqual] [>=]; [Lower] [<]; [LowerOrEqual] [<=];
)]
impl ord {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::exec(lhs, rhs).into(),
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match_any! { (lhs, rhs),
            (Variable::Int(lhs), Variable::Int(rhs)) | (Variable::Float(lhs), Variable::Float(rhs))
                => (lhs op rhs).into(),
            (lhs, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|rhs| Self::exec(lhs.clone(), rhs))
                .collect(),
            (Variable::Array(array), rhs) => array
                .iter()
                .cloned()
                .map(|lhs| Self::exec(lhs, rhs.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} {} {rhs}", stringify!(op))
        }
    }
}

#[duplicate_item(
    ord;
    [Greater]; [GreaterOrEqual]; [Lower]; [LowerOrEqual];
)]
impl ReturnType for ord {
    fn return_type(&self) -> Type {
        let lhs = self.lhs.return_type();
        let rhs = self.rhs.return_type();
        return_type(lhs, rhs)
    }
}

pub fn return_type(lhs: Type, rhs: Type) -> Type {
    if lhs.matches(&[Type::Any].into()) || rhs.matches(&[Type::Any].into()) {
        return [Type::Int].into();
    }
    if Type::from([Type::Never]).matches(&lhs) || Type::from([Type::Never]).matches(&rhs) {
        return [Type::Int] | Type::Int;
    }
    Type::Int
}

#[cfg(test)]
mod tests {
    use crate::{instruction::ord::return_type, variable::Type};

    #[test]
    fn test_return_type() {
        assert_eq!(return_type(Type::Int, Type::Int), Type::Int);
        assert_eq!(return_type(Type::Float, Type::Float), Type::Int);
        assert_eq!(
            return_type([Type::Int].into(), Type::Int),
            [Type::Int].into()
        );
        assert_eq!(
            return_type([Type::Float].into(), Type::Float),
            [Type::Int].into()
        );
        assert_eq!(
            return_type(Type::Float, [Type::Float].into()),
            [Type::Int].into()
        );
        assert_eq!(
            return_type(Type::Int, [Type::Int] | Type::Int),
            [Type::Int] | Type::Int
        );
        assert_eq!(
            return_type(Type::Float, [Type::Float] | Type::Float),
            [Type::Int] | Type::Int
        );
    }
}
