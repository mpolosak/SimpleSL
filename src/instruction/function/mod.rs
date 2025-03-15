mod anonymous;
pub mod call;
mod declaration;
pub use self::{anonymous::AnonymousFunction, declaration::FunctionDeclaration};
