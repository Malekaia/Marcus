mod helpers;
mod core;
use crate::helpers::fileio;
use std::collections::HashMap;

fn main() {
  // Glob the MD test files
  for file_path in fileio::walk("./test/**/*.md") {
    // Read the file's contents
    let mut html: String = fileio::read_file(&file_path);

    // Call parser methods
    let ignore: HashMap<i32, String> = core::inline_ignore::hide(&mut html);
    core::escape::default(&mut html);
    core::comments::default(&mut html);
    core::emojis::default(&mut html);
    core::blockquotes::default(&mut html);
    core::code::default(&mut html);
    core::emphasis::default(&mut html);
    core::headings::default(&mut html);
    core::horizontal_rule::default(&mut html);
    core::links_images_footnotes::default(&mut html);
    core::lists_and_task_lists::default(&mut html);
    core::highlight::default(&mut html);
    core::strikethrough::default(&mut html);
    core::subscript::default(&mut html);
    core::superscript::default(&mut html);
    core::auto_link::default(&mut html);
    core::definition_list::default(&mut html);
    core::table::default(&mut html);
    core::paragraphs::default(&mut html);
    core::inline_ignore::show(&mut html, ignore);

    // Write to the test file
    fileio::write_file(&file_path.replace(".md", ".html"), html);
  }
}
