pub mod escape;
pub mod comments;
pub mod blockquotes;
pub mod inline_code;
pub mod emphasis;
pub mod headings;

use regex::{Captures, Regex};

pub fn replacer<F: Fn(Captures) -> (String, String)>(html: &mut String, regex: &str, method: F) {
  // Create the regex
  let re: Regex = Regex::new(regex).unwrap();
  // Check for headings with ID
  if re.is_match(&html) {
    // Iter all captures
    let parsed: Vec<(String, String)> = re.captures_iter(&html)
      .map(| capture: Captures | -> (String, String) { method(capture) })
      .collect::<Vec<(String, String)>>();
    // Replace the parsed capture with it's replacement
    for (capture, replacement) in parsed {
      *html = html.replace(&capture, &replacement);
    }
  }
}
