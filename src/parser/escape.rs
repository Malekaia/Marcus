use crate::regex::RE;
use html_escape;
use regex::Regex;
use std::borrow::Cow;

pub fn default(html: &mut String) {
  // Create the regex
  let re: Regex = Regex::new(RE::ESCAPE).unwrap();
  // Check for escaped characters
  if re.is_match(html) {
    // Iter all captures
    for capture in re.captures_iter(html.clone().as_ref()) {
      // HTML encode the escaped character (or return the raw character if not required)
      let replacement: Cow<str> = html_escape::encode_safe(&capture[1]);
      // Replace the raw escape with the encoded HTML
      *html = html.replace(&capture[0], &replacement);
    }
  }
}
