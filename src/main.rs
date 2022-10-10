mod core;
mod parser;
use crate::core::fileio;
use std::collections::HashMap;

fn main() {
  // Glob the MD test files
  for file_path in fileio::walk("./test/**/*.md") {
    // Read the file's contents
    let mut html: String = fileio::read_file(&file_path);

    // Call parser methods
    let ignore: HashMap<i32, String> = parser::inline_ignore::hide(&mut html);
    parser::escape::default(&mut html);
    parser::comments::default(&mut html);
    parser::emojis::default(&mut html);
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
    parser::definition_list::default(&mut html);
    parser::table::default(&mut html);
    parser::paragraphs::default(&mut html);
    parser::inline_ignore::show(&mut html, ignore);

    // Write to the test file
    fileio::write_file(&file_path.replace(".md", ".html"), html);
  }
}
