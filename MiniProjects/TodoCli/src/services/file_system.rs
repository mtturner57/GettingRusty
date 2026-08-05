use std::fs::File;
use std::io::{self, Write, BufReader, Read};
use std::path::Path;

pub fn check_exists() -> io::Result<bool> {
    Path::new("./task_list.txt").try_exists()
}

pub fn create_file() -> io::Result<()> {
    let mut file = File::create("./task_list.txt")?;
    file.write_all(b"[]")?;
    Ok(())
}

pub fn get_task_content() -> io::Result<String> {
    let file = File::open("./task_list.txt")?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn create_task(newTask: String) -> io::Result<String> {
    let file = get_task_content();
}