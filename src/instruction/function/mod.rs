mod anonymous;
mod call;
mod declaration;

pub use self::{
    anonymous::AnonymousFunction, call::FunctionCall, declaration::FunctionDeclaration,
};
