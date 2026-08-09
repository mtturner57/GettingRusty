use std::fs;
use std::fs::File;
use crate::Task;
use std::io::{self, Write, BufReader, Read};
use std::path::Path;

pub fn check_exists() -> io::Result<bool> {
    Path::new("./task_list.json").try_exists()
}

pub fn create_file() -> io::Result<()> {
    let mut file = File::create("./task_list.json")?;
    file.write_all(b"[]")?;
    Ok(())
}

pub fn get_task_content() -> io::Result<String> {
    let file = File::open("./task_list.json")?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn create_task(tasks: &Vec<Task>) -> io::Result<()> {
    let json = serde_json::to_string_pretty(&tasks)?;
    fs::write("task_list.json", json)?;

    Ok(())
}