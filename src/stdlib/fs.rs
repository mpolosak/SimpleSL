use crate as simplesl;
use simplesl_macros::export;
use std::{fs, io};

#[export]
mod add_fs {
    fn file_read_to_string(path: &str) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn write_to_file(path: &str, contents: &str) -> io::Result<()> {
        fs::write(path, contents)
    }

    fn copy_file(from: &str, to: &str) -> io::Result<()> {
        fs::copy(from, to)?;
        Ok(())
    }

    fn remove_file(path: &str) -> io::Result<()> {
        fs::remove_file(path)
    }

    fn remove_dir(path: &str) -> io::Result<()> {
        fs::remove_dir(path)
    }

    fn remove_dir_all(path: &str) -> io::Result<()> {
        fs::remove_dir_all(path)
    }

    fn create_dir(path: &str) -> io::Result<()> {
        fs::create_dir(path)
    }

    fn create_dir_all(path: &str) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn rename(from: &str, to: &str) -> io::Result<()> {
        fs::rename(from, to)
    }
}
