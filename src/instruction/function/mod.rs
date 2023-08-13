mod anonymous;
mod call;
mod function_declaration;

pub use self::{
    anonymous::AnonymousFunction, call::FunctionCall, function_declaration::FunctionDeclaration,
};
