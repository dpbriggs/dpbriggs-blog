use crate::blog::ParsingError;
use miette::Diagnostic;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum SiteError {
    #[error(transparent)]
    #[diagnostic(code(app::io_error))]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(app::tera_error))]
    TeraError(#[from] tera::Error),

    #[error(transparent)]
    #[diagnostic(code(app::fs_extra_error))]
    FsExtraError(#[from] fs_extra::error::Error),

    #[error("File not found: {0}")]
    #[diagnostic(code(app::file_not_found))]
    FileNotFound(String),

    #[error(transparent)]
    #[diagnostic(code(app::parsing_error))]
    ParsingError(#[from] ParsingError),

    #[error("Not a directory: {0}")]
    #[diagnostic(code(app::not_a_directory))]
    NotADirectory(PathBuf),
}