pub mod escape;
pub mod comments;
pub mod blockquotes;
pub mod inline_code;
pub mod emphasis;
pub mod headings;
pub mod horizontal_rule;
pub mod links_images_footnotes;
pub mod lists;

use regex::{Captures, Regex};

// TODO: predefine regex
pub fn replacer<F: FnMut(Captures) -> String>(html: &mut String, regex: &str, mut handler: F) {
  // Create the specified regular expression
  let re: Regex = Regex::new(regex).unwrap();
  // Ignore non-matches
  if re.is_match(&html) {
    // Insert full capture and parsed HTML into the result vector
    let mut result: Vec<(String, String)> = vec![];
    for capture in re.captures_iter(&html) {
      result.push((capture[0].to_string(), handler(capture)));
    }
    // Replace the MD capture with the parsed HTML
    for (capture, parsed) in result {
      *html = html.replace(&capture, &parsed);
    }
  }
}
