use crate as simplesl;
use simplesl_macros::decls;

decls! {
    AND:=(iter: () -> (bool, int)) -> int {
        return iter $!0 (acc: int, curr: int) -> int {
            return acc & curr;
        }
    }
    OR:=(iter: () -> (bool, int)) -> int {
        return iter $0 (acc: int, curr: int) -> int {
            return acc | curr;
        }
    }
    ALL:=(iter: () -> (bool, bool)) -> bool {
        for value in iter {
            if !value { return false; }
        }
        return true;
    }
    ANY:=(iter: () -> (bool, bool)) -> bool {
        for value in iter {
            if value { return true; }
        }
        return false;
    }
    INT_PRODUCT:=(iter: () -> (bool, int)) -> int {
        return iter $1 (acc: int, curr: int) -> int {
            return acc * curr;
        }
    }
    FLOAT_PRODUCT:=(iter: () -> (bool, float)) -> float {
        return iter $1.0 (acc: float, curr: float) -> float {
            return acc * curr;
        }
    }
    INT_SUM:=(iter: () -> (bool, int)) -> int {
        return iter $0 (acc: int, curr: int) -> int {
            return acc + curr;
        }
    }
    FLOAT_SUM:=(iter: () -> (bool, float)) -> float {
        return iter $0.0 (acc: float, curr: float) -> float {
            return acc + curr;
        }
    }
    STRING_SUM:=(iter: () -> (bool, string)) -> string {
        return iter $"" (acc: string, curr: string) -> string {
            return acc + curr;
        }
    }
    REDUCE:=(iter: () -> (bool, any), initial_value: any, function: (any, any) -> any) -> any {
        result:=mut initial_value;
        for element in iter {
            result=function(*result, element)
        }
        return *result
    }
    FILTER:=(iter: () -> (bool, any), predicate: (any) -> bool) -> () -> (bool, any) {
        return () -> (bool, any) {
            loop {
                res := iter();
                (con, value) := res;
                if !con || predicate(value) return res;
            }
            return (false, 0);
        }
    }
    Operators:=struct{
        bitand_reduce := AND,
        bitor_reduce := OR,
        all := ALL,
        any := ANY,
        int_product := INT_PRODUCT,
        float_product := FLOAT_PRODUCT,
        int_sum := INT_SUM,
        float_sum := FLOAT_SUM,
        string_sum := STRING_SUM,
        reduce := REDUCE,
        filter := FILTER
    }
}
