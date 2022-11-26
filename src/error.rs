use std::io;
use std::path::PathBuf;
use thiserror::Error;

pub type Ganzu<A> = std::result::Result<A, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("error reading input: {0:?}")]
    InputFailure(io::Error),
    #[error("failed to canonicalize path: {0:?}")]
    CanonicalizeFailed(io::Error),
    #[error("failed to open file: {error:?}")]
    OpenFailed { path: PathBuf, error: nix::Error },
    #[error("failed to stat file: {error:?}")]
    StatFailed { path: PathBuf, error: nix::Error },
}
