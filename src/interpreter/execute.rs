//! Contains all the runtime logic needed for executing the AST.

pub use executing::Executor;

mod definitions;
mod executing;
mod inbuilt;
mod operations;
mod state;
