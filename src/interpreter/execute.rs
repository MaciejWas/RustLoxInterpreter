//! Contains all the runtime logic needed for executing the AST.

pub use executing::Executor;

mod definitions;
mod executing;
mod operations;
mod state;
mod inbuilt;