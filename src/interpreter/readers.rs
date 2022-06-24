//! Utility class for reading from a stream of Tokens or characters without mutable state.

pub use reader::{Reader, ReaderBase};
pub use text_reader::TextReader;
pub use token_reader::TokenReader;

pub mod reader;
pub mod text_reader;
pub mod token_reader;
