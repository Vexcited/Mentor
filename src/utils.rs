pub fn find_between(content: &str, start: &str, end: &str) -> String {
  let start_index = content.find(start).unwrap();
  let start_index = start_index + start.len();

  let end_index = content[start_index..].find(end).unwrap();
  let end_index = start_index + end_index;

  content[start_index..end_index].to_string()
}
