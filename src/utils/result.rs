use std::{fmt::{Display, Formatter, Result as FmtResult}, io::Error as IoError, path::PathBuf};
use globset::Error as GlobError;
use thiserror::Error;

/// This enum represents an error
/// for all the functions exported by the `fdrs`
/// library.
#[derive(Error, Debug)]
pub enum FindError {
    /// This error is returned when the specified path
    /// is not a directory, as you can't find files in a file.
    #[error("The path {0} is a directory.")]
    NotADirectory(String),

    /// This is returned when a general I/O error happens,
    /// permissions errors and file iteration errors
    /// are not included here, but in the `DirectoryEntry`
    /// as a literal string.
    #[error("I/O Error: {0:#}")]
    IoError(#[from] IoError),

    /// This is returned when there is a glob compilation error.
    #[error("Glob compilation error: {0:#}")]
    GlobCompile(#[from] GlobError)
}

/// This represents an ok result for all
/// the functions exported by the `fdrs`
/// library. Usually contained in a Vec.
pub enum DirectoryEntry {
    /// This result contains the path to the
    /// file found.
    Ok(PathBuf),

    /// This result contains a path reading
    /// error such as missing permissions.
    Error(String)
}

impl Display for DirectoryEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", match self {
            DirectoryEntry::Ok(entry) => entry
                .to_string_lossy()
                .to_string(),
            DirectoryEntry::Error(error) => error.clone()
        })
    }
}
