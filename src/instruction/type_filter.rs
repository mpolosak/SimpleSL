use super::InstructionWithStr;
use crate::variable::Variable;
use crate::{self as simplesl, BinOperator, Code};
use crate::{
    instruction::{local_variable::LocalVariables, Exec, ExecResult, Instruction, Recreate},
    interpreter::Interpreter,
    variable::{ReturnType, Type},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct TypeFilter {
    iterator: InstructionWithStr,
    var_type: Type,
}

impl TypeFilter {
    pub fn create_instruction(
        iterator: InstructionWithStr,
        var_type: Pair<Rule>,
    ) -> Result<Instruction, Error> {
        let array_type = iterator.return_type();
        let var_type = Type::from(var_type);
        if array_type.iter_element().is_none() {
            return Err(Error::CannotDo2(array_type, BinOperator::Filter, var_type));
        }
        Ok(Self { iterator, var_type }.into())
    }
}

impl Exec for TypeFilter {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iterator = self.iterator.exec(interpreter)?;
        let mut interpreter = interpreter.create_layer();
        interpreter.insert("iterator".into(), iterator);
        let default_value = Variable::of_type(&self.var_type).unwrap();
        interpreter.insert("default".into(), default_value);
        Ok(Code::parse(
            &interpreter,
            &format!(
                "() -> (bool, {}) {{
                    loop {{
                        res := iterator();
                        (con, value) := res;
                        if !con return (false, default);
                        if value:{0} = value return (true, value);
                    }}
                    return (false, default);
                }}",
                self.var_type
            ),
        )
        .unwrap()
        .exec()?)
    }
}

impl Recreate for TypeFilter {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let iterator = self.iterator.recreate(local_variables)?;
        let var_type = self.var_type.clone();
        Ok(Self { iterator, var_type }.into())
    }
}

impl ReturnType for TypeFilter {
    fn return_type(&self) -> Type {
        let iter_type = self.var_type.clone();
        var_type!(() -> (bool, iter_type))
    }
}
