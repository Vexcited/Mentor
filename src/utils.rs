use anyhow::Result;
use std::{
  fs::{File, OpenOptions},
  io::{self, Read, Seek, SeekFrom, Write},
  path::Path,
};

pub fn find_between(content: &str, start: &str, end: &str) -> String {
  let start_index = content.find(start).unwrap();
  let start_index = start_index + start.len();

  let end_index = content[start_index..].find(end).unwrap();
  let end_index = start_index + end_index;

  content[start_index..end_index].to_string()
}

pub fn open_readme() -> io::Result<File> {
  open_file("README.md")
}

pub fn file_exists(path: &str) -> bool {
  Path::new(path).exists()
}

pub fn open_file(path: &str) -> io::Result<File> {
  OpenOptions::new()
    .read(true)
    .write(true)
    .truncate(false)
    .create(true)
    .open(path)
}

pub fn read_file(file: &mut File) -> Result<String> {
  let mut buffer = String::new();
  file.read_to_string(&mut buffer)?;

  Ok(buffer)
}

pub fn write_file(file: &mut File, content: String) -> Result<()> {
  // Move cursor to the beginning.
  file.seek(SeekFrom::Start(0))?;
  file.set_len(0)?;

  // Write the content to the file.
  file.write_all(content.as_bytes())?;
  file.flush()?;

  Ok(())
}
