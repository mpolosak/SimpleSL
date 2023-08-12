use crate::{
    instruction::{Exec, Instruction},
    interpreter::Interpreter,
    variable::{GetReturnType, Type, Variable},
    Result,
};

#[derive(Debug)]
pub enum Body {
    Lang(Box<[Instruction]>),
    Native {
        body: fn(&mut Interpreter) -> Result<Variable>,
        return_type: Type,
    },
}

impl Exec for Body {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        match self {
            Body::Lang(body) => interpreter.exec(body),
            Body::Native { body, .. } => (body)(interpreter),
        }
    }
}

impl GetReturnType for Body {
    fn get_return_type(&self) -> Type {
        match self {
            Body::Lang(body) => match body.last() {
                Some(instruction) => instruction.get_return_type(),
                None => Type::Void,
            },
            Body::Native { return_type, .. } => return_type.clone(),
        }
    }
}
