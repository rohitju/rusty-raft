use std::fs::File;
use std::path::PathBuf;

/// Abstraction of THE LOG. 
/// Functions to write log entries to disk.
/// Log entries are protobuf messages.

pub struct Log {
    path: PathBuf,  
}

impl Log {
    pub fn new(filepath: String) -> Log {
        let path = PathBuf::from(filepath);
        Log{ path:path } 
    }

}
