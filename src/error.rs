use thiserror::Error;
use tree_sitter::LanguageError;
use std::io::Error as IOError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("The language is incompatible")]
    IncompatibleLanguage(#[from] LanguageError),
    #[error("File loading went wrong")]
    IOError(#[from] IOError)
}
