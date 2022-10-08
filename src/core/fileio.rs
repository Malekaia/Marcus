use glob::glob;
use std::fs;

pub fn read_file(file_path: &str) -> String {
  // Read the file
  match fs::read_to_string(file_path) {
    // Return the file content
    Ok(file_content) => file_content,
    // Output error
    Err(error_message) => panic!("Marcus: ReadError: failed to read {}\n\n{}", &file_path, &error_message)
  }
}

pub fn write_file(file_path: &str, contents: String) {
  fs::write(file_path, contents).expect("Marcus: WriteError: failed to write to file");
}

pub fn walk(path: &str) -> impl Iterator<Item = String> {
  glob(path)
    .expect("Marcus: GlobError: Failed to read glob pattern")
    .map(| entry |
      entry.expect("Marcus: Error: failed to glob entry").display().to_string()
    )
}
