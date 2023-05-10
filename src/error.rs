use std::{
    error::Error as StdError,
    fmt::{Debug, Display},
    io,
    result::Result as StdResult,
};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    PathNotFound,
    LibraryUnloaded,
    IoFailure(io::Error),
    LoadLibraryError(libloading::Error),
    LoadSymbolError(libloading::Error),
    FileWatcherError(notify::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            PathNotFound => write!(f, "Path could not be found."),
            LibraryUnloaded => write!(f, "Library has been unloaded and cannot be used."),
            IoFailure(e) => write!(f, "File system IO error.").and(Display::fmt(e, f)),
            LoadLibraryError(e) => write!(f, "Could not load library.").and(Display::fmt(e, f)),
            LoadSymbolError(e) => write!(f, "Could not load symbol.").and(Display::fmt(e, f)),
            FileWatcherError(e) => write!(f, "File watcher error.").and(Display::fmt(e, f)),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;
        match self {
            LoadLibraryError(e) | LoadSymbolError(e) => e.source(),
            FileWatcherError(e) => e.source(),
            _ => None,
        }
    }
}
