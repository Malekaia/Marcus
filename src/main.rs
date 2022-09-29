mod parser;
mod regex;
use std::fs;

fn read_file(file_path: &str) -> String {
  // Read the file
  match fs::read_to_string(file_path) {
    // Return the file content
    Ok(file_content) => file_content,
    // Output error
    Err(error_message) => panic!("Marcus: ReadError: failed to read {}\n\n{}", &file_path, &error_message)
  }
}

fn main() {
  // Options
  const ALLOW_COMMENTS: bool = false;
  // Read MD file
  let mut html: String = read_file("./test/basic/blockquote.md");
  // Parser methods
  parser::escape::default(&mut html);
  parser::comments::default(&mut html, ALLOW_COMMENTS);
  parser::blockquotes::default(&mut html);
  parser::inline_code::default(&mut html);
  parser::emphasis::default(&mut html);
  parser::headings::default(&mut html);
  // Debug
  println!("{html}");
}
