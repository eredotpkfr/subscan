use std::fs::File;

use derive_more::From;

/// Output file type, it stores file name and object
#[derive(From)]
pub struct OutputFile {
    /// File name
    pub name: String,
    /// File object to write or read something
    pub descriptor: File,
}
