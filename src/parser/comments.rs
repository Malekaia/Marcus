use crate::regex::RE;
use regex::Regex;

pub fn default(html: &mut String, allow_comments: bool) {
  // Create the escape regex
  let re: Regex = Regex::new(RE::COMMENTS).unwrap();
  // Check for escaped characters
  if re.is_match(html) && allow_comments == false {
    // Iter all captures
    for capture in re.captures_iter(html.clone().as_ref()) {
      // Remove the comment if requested
      *html = html.replace(&capture[0], "");
    }
  }
}
