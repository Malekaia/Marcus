mod helpers;
mod core;
use std::collections::HashMap;

// Get a HTML string from a MD file
pub fn to_string(md: String) -> String {
  // Read the file's contents
  let mut html: String = md;
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
  // Return the output HTML
  html
}
