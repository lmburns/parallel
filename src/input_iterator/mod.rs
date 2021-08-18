mod iterator;
mod lock;

pub use self::{
    iterator::{InputIterator, ETA},
    lock::InputsLock,
};

use std::{io, path::PathBuf};

/// The `InputIterator` may possibly encounter an error with reading from the
/// unprocessed file.
#[derive(Debug)]
pub enum InputIteratorErr {
    FileRead(PathBuf, io::Error),
}
