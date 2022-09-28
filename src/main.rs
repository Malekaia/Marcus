mod parser;
mod regex;
use std::fs;

// File read util
fn read_file(file_path: &str) -> String {
  fs::read_to_string(file_path)
     .expect("Should have been able to read the file")
}

fn main() {
  // OPTIONS
  const ALLOW_COMMENTS: bool = false;

  // READ MD FILE
  let mut html: String = read_file("./test/extended/heading-id.md");

  // PARSER METHODS
  parser::escape::default(&mut html);
  parser::comments::default(&mut html, ALLOW_COMMENTS);
  parser::blockquotes::default(&mut html);
  parser::inline_code::default(&mut html);
  parser::emphasis::default(&mut html);
  parser::headings::default(&mut html);

  // DEBUG
  println!("{html}");
}
