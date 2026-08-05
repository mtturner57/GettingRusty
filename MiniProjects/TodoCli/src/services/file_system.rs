use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn check_exists() -> io::Result<bool> {
    Path::new("./task_list.txt").try_exists()
}

pub fn create_file() -> io::Result<()> {
    let mut file = File::create("./task_list.txt")?;
    file.write_all(b"[]")?;
    Ok(())
}

pub fn list() {
    // ...
}