use crate as simplesl;
use simplesl_macros::export;

#[export(FS)]
mod inner {
    use std::fs;
    pub use std::io;
    pub fn file_read_to_string(path: &str) -> io::Result<String> {
        fs::read_to_string(path)
    }

    pub fn write_to_file(path: &str, contents: &str) -> io::Result<()> {
        fs::write(path, contents)
    }

    pub fn copy_file(from: &str, to: &str) -> io::Result<()> {
        fs::copy(from, to)?;
        Ok(())
    }

    pub fn remove_file(path: &str) -> io::Result<()> {
        fs::remove_file(path)
    }

    pub fn remove_dir(path: &str) -> io::Result<()> {
        fs::remove_dir(path)
    }

    pub fn remove_dir_all(path: &str) -> io::Result<()> {
        fs::remove_dir_all(path)
    }

    pub fn create_dir(path: &str) -> io::Result<()> {
        fs::create_dir(path)
    }

    pub fn create_dir_all(path: &str) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    pub fn rename(from: &str, to: &str) -> io::Result<()> {
        fs::rename(from, to)
    }
}
