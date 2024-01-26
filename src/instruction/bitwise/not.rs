use crate::prefixOp;

prefixOp!(BitwiseNot, "~", int, |num: i64| !num);
