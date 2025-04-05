use anyhow::Result;
use std::{fs::File, io, io::Read};

pub fn find_between(content: &str, start: &str, end: &str) -> String {
  let start_index = content.find(start).unwrap();
  let start_index = start_index + start.len();

  let end_index = content[start_index..].find(end).unwrap();
  let end_index = start_index + end_index;

  content[start_index..end_index].to_string()
}

pub fn open_readme() -> io::Result<File> {
  File::open("README.md")
}

pub fn read_file(file: &mut File) -> Result<String> {
  let mut buffer = String::new();
  file.read_to_string(&mut buffer)?;

  Ok(buffer)
}
