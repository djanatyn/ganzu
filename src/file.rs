use nix::fcntl;
use nix::sys::stat::{fstat, Mode};
use std::path::PathBuf;

use crate::error::{Error, Ganzu};

/// Snapshot of file state.
#[derive(Debug, Clone)]
pub struct FileSnapshot {
    /// Initial name provided when searching for file.
    pub input_name: String,
    /// Absolute path, returned by canonicalize() method.
    pub absolute_path: PathBuf,
    /// Last modification time.
    pub last_mtime: i64,
    /// Mimetype returned with tree_magic::from_filepath.
    pub mimetype: String,
}

impl FileSnapshot {
    /// Snapshot a potential path to a file provided by the user.
    pub fn new(filename: &str) -> Ganzu<FileSnapshot> {
        let absolute_path = PathBuf::from(filename)
            .canonicalize()
            .map_err(Error::CanonicalizeFailed)?;

        let fd = fcntl::open(&absolute_path, fcntl::OFlag::O_RDONLY, Mode::S_IRUSR).map_err(
            |error| Error::OpenFailed {
                path: absolute_path.clone(),
                error,
            },
        )?;

        let stat = fstat(fd).map_err(|error| Error::StatFailed {
            path: absolute_path.clone(),
            error,
        })?;

        let mimetype = tree_magic::from_filepath(&absolute_path);

        Ok(FileSnapshot {
            input_name: filename.into(),
            absolute_path,
            last_mtime: stat.st_mtime,
            mimetype,
        })
    }
}
