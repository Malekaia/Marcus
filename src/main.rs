// TODO: convert program to library
mod core;
mod parser;
use crate::core::fileio;

// FIXME: move to external mod
pub struct Options {
  pub allow_comments: bool
}

// TODO: convert to library function
fn main() {
  // Create options struct
  let options: Options = Options {
    allow_comments: false
  };

  // Glob the MD test files
  for file_path in fileio::walk("./test/**/*.md") {
    // Read the file's contents
    let mut html: String = fileio::read_file(&file_path);

    // Call parser methods
    parser::escape::default(&mut html);
    parser::comments::default(&mut html, options.allow_comments); // FIXME: Provide access to all modules
    parser::blockquotes::default(&mut html);
    parser::code::default(&mut html);
    parser::emphasis::default(&mut html);
    parser::headings::default(&mut html);
    parser::horizontal_rule::default(&mut html);
    parser::links_images_footnotes::default(&mut html);
    parser::lists_and_task_lists::default(&mut html);
    parser::highlight::default(&mut html);
    parser::strikethrough::default(&mut html);
    parser::subscript::default(&mut html);
    parser::superscript::default(&mut html);
    parser::auto_link::default(&mut html);

    // Write to the test file
    fileio::write_file(&file_path.replace(".md", ".html"), html);
  }
}
