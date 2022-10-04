mod parser;
mod regex;
use std::{fs, path::PathBuf};
use glob::{glob, GlobError};

pub struct Options {
  pub allow_comments: bool
}

fn read_file(file_path: &str) -> String {
  // Read the file
  match fs::read_to_string(file_path) {
    // Return the file content
    Ok(file_content) => file_content,
    // Output error
    Err(error_message) => panic!("Marcus: ReadError: failed to read {}\n\n{}", &file_path, &error_message)
  }
}

fn write_file(file_path: &str, contents: String) {
  fs::write(file_path, contents).expect("Marcus: WriteError: failed to write to file");
}

fn main() {
  // Create options struct
  let options: Options = Options {
    allow_comments: false
  };

  // Glob the test folder
  glob("./test/**/*.md")
    // Handle errors
    .expect("Marcus: GlobError: Failed to read glob pattern")
    // Iterate globbed paths
    .for_each(| entry: Result<PathBuf, GlobError> | {
      // Get the file path and read the file's contents
      let file_path: String = entry.expect("Marcus: Error: failed to glob entry").display().to_string();
      let mut html: String = read_file(&file_path);
      // Call parser methods
      parser::escape::default(&mut html);
      parser::comments::default(&mut html, options.allow_comments);
      parser::blockquotes::default(&mut html);
      parser::inline_code::default(&mut html);
      parser::emphasis::default(&mut html);
      parser::headings::default(&mut html);
      parser::horizontal_rule::default(&mut html);
      // Write to the test file
      write_file(&file_path.replace(".md", ".html"), html);
    });
}
